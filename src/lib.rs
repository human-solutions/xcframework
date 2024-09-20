#![doc = include_str!("../README.md")]
mod cmd;
mod conf;
pub mod core;
pub mod ext;

use core::platform::{ApplePlatform, Environment};
use std::collections::HashMap;

use crate::conf::Configuration;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use cmd::{cargo, rustup};
pub use conf::{XCFrameworkConfiguration, XcCli};
use ext::PathBufExt;
use fs_err as fs;
use rustup_configurator::target::Triple;

#[derive(Debug, PartialEq, Eq)]
pub struct Produced {
    pub module_name: String,
    pub path: Utf8PathBuf,
    pub is_zipped: bool,
}

pub fn build(cli: XcCli) -> Result<Produced> {
    let conf = Configuration::load(cli).context("loading configuration")?;

    conf.build_dir
        .remove_dir_all_if_exists()
        .context("cleaning build dir")?;

    rustup::check_needed(&conf).context("checking rustup targets")?;
    cargo::build(&conf).context("running cargo build")?;

    let libs = {
        let conf = &conf;
        let libs_dir = conf.build_dir.join("libs");
        fs::create_dir_all(&libs_dir)?;

        let mut platform_lib_paths = HashMap::new();
        if conf.cargo_section.iOS {
            let lib_paths = lib_paths_for_targets(conf, &conf.cargo_section.iOS_targets)?;
            platform_lib_paths.insert(ApplePlatform::IOS(Environment::Device), lib_paths);
        }
        if conf.cargo_section.simulators {
            let lib_paths = lib_paths_for_targets(conf, &conf.cargo_section.iOS_simulator_targets)?;
            platform_lib_paths.insert(ApplePlatform::IOS(Environment::Simulator), lib_paths);
        }
        if conf.cargo_section.macOS {
            let lib_paths = lib_paths_for_targets(conf, &conf.cargo_section.macOS_targets)?;
            platform_lib_paths.insert(ApplePlatform::MacOS, lib_paths);
        }

        let ending = conf.lib_type.file_ending();
        let name = &conf.lib_name.replace('-', "_");
        let output_lib_name = format!("lib{name}.{ending}");

        crate::core::lipo_create_platform_libraries(
            &platform_lib_paths,
            &output_lib_name,
            &libs_dir,
        )
    }
    .context("lipo: assembling libraries")?;

    let bundle_name = conf.module_name()?;
    let crate_type = match conf.lib_type {
        conf::LibType::StaticLib => &core::CrateType::Staticlib,
        conf::LibType::CDyLib => &core::CrateType::Cdylib,
    };
    let framework_paths = libs
        .into_iter()
        .map(|(platform, lib_path)| {
            let include_dir = &conf.cargo_section.include_dir;
            let header_paths = get_header_paths(include_dir)?;
            let module_paths = get_module_paths(include_dir)?;
            let frameworks_dir = conf.target_dir.join("frameworks");
            std::fs::create_dir_all(&frameworks_dir)?;

            core::wrap_as_framework(
                platform,
                crate_type,
                &lib_path,
                header_paths,
                module_paths,
                &bundle_name,
                &frameworks_dir,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let xcframework_path =
        crate::core::create_xcframework(framework_paths, &conf.module_name()?, &conf.build_dir)?;
    let module_name = conf.module_name()?;

    let (path, is_zipped) = if conf.cargo_section.zip {
        (
            core::compress_xcframework(None, &xcframework_path, None, &conf.target_dir)?,
            true,
        )
    } else {
        let to = conf.target_dir.join(format!("{module_name}.xcframework"));
        if to.exists() {
            fs::remove_dir_all(&to)?;
        }
        fs::rename(xcframework_path, &to)?;
        (to, false)
    };

    conf.build_dir.remove_dir_all_if_exists()?;

    Ok(Produced {
        module_name,
        path,
        is_zipped,
    })
}

fn get_header_paths(include_dir: &Utf8PathBuf) -> Result<Vec<Utf8PathBuf>> {
    let mut header_paths = Vec::new();
    let pattern = format!("{}/**/*.h", include_dir);

    for entry in glob::glob(&pattern)? {
        match entry {
            Ok(path) => header_paths.push(Utf8PathBuf::from_path_buf(path).unwrap()),
            Err(e) => println!("{:?}", e),
        }
    }

    Ok(header_paths)
}

fn get_module_paths(include_dir: &Utf8PathBuf) -> Result<Vec<Utf8PathBuf>> {
    let mut module_paths = Vec::new();
    let pattern = format!("{}/**/*.modulemap", include_dir);
    for entry in glob::glob(&pattern)? {
        match entry {
            Ok(path) => module_paths.push(Utf8PathBuf::from_path_buf(path).unwrap()),
            Err(e) => println!("{:?}", e),
        }
    }
    Ok(module_paths)
}

fn lib_paths_for_targets(conf: &Configuration, targets: &[Triple]) -> Result<Vec<Utf8PathBuf>> {
    let mut paths = vec![];

    let target_dir = &conf.target_dir;
    let profile = conf.profile();
    let ending = conf.lib_type.file_ending();
    let name = &conf.lib_name.replace('-', "_");

    for target in targets {
        let path = target_dir
            .join(target)
            .join(profile)
            .join(format!("lib{name}.{ending}"));
        paths.push(path)
    }
    Ok(paths)
}
