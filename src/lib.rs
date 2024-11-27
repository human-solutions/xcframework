#![doc = include_str!("../README.md")]
mod cmd;
mod conf;
pub mod core;

use core::platform::{ApplePlatform, Environment};
use std::collections::HashMap;

pub use crate::conf::Configuration;
use anyhow::{Context, Result};
use camino_fs::*;
use cmd::cargo;
pub use conf::CliArgs;
use conf::Target;
pub use conf::{LibType, XCFrameworkConfiguration};

#[derive(Debug, PartialEq, Eq)]
pub struct Produced {
    pub module_name: String,
    pub path: Utf8PathBuf,
    pub is_zipped: bool,
}

pub fn build_from_cli(cli: CliArgs) -> Result<Produced> {
    let config = Configuration::load(cli).context("loading configuration")?;

    crate::build(&config)
}

pub fn build(conf: &Configuration) -> Result<Produced> {
    conf.build_dir.rm().context("cleaning build dir")?;

    cargo::build(&conf).context("running cargo build")?;

    let libs = {
        let conf = &conf;
        let libs_dir = conf.build_dir.join("libs");
        libs_dir.mkdirs()?;

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

    let bundle_name = conf.module_name().context("finding module name")?;

    let crate_type = match conf.lib_type {
        conf::LibType::StaticLib => &core::CrateType::Staticlib,
        conf::LibType::CDyLib => &core::CrateType::Cdylib,
    };

    let framework_paths = libs
        .into_iter()
        .map(|(platform, lib_path)| {
            let include_dir = &conf.cargo_section.include_dir;
            let header_paths = get_header_paths(include_dir)?;
            let module_path = get_module_path(include_dir)?;
            let frameworks_dir = conf.target_dir.join("frameworks");
            frameworks_dir.mkdirs()?;

            core::wrap_as_framework(
                platform,
                crate_type,
                &lib_path,
                header_paths,
                module_path,
                &bundle_name,
                &frameworks_dir,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()
        .context("collecting framework paths")?;

    let xcframework_path =
        crate::core::create_xcframework(framework_paths, &conf.module_name()?, &conf.build_dir)
            .context("creating xcframework")?;

    let module_name = conf.module_name()?;

    let path = if conf.cargo_section.zip {
        core::compress_xcframework(None, &xcframework_path, None, &conf.target_dir)?
    } else {
        let to = conf.target_dir.join(format!("{module_name}.xcframework"));
        to.rm()?;
        xcframework_path.mv(&to)?;
        to
    };

    conf.build_dir.rm().context("cleaning build dir")?;

    Ok(Produced {
        module_name,
        path,
        is_zipped: conf.cargo_section.zip,
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

fn get_module_path(include_dir: &Utf8PathBuf) -> Result<Utf8PathBuf> {
    let pattern = format!("{include_dir}/**/*.modulemap");
    let mut glob = glob::glob(&pattern)?;
    let module_path = glob.next().context("modulemap not found")??;
    if glob.next().is_some() {
        anyhow::bail!("multiple modulemaps found");
    }

    Ok(Utf8PathBuf::from_path_buf(module_path).unwrap())
}

fn lib_paths_for_targets(conf: &Configuration, targets: &[Target]) -> Result<Vec<Utf8PathBuf>> {
    let mut paths = vec![];

    let target_dir = &conf.target_dir;
    let profile = conf.profile();
    let ending = conf.lib_type.file_ending();
    let name = &conf.lib_name.replace('-', "_");

    for target in targets {
        let path = target_dir
            .join(target.as_str())
            .join(profile)
            .join(format!("lib{name}.{ending}"));
        paths.push(path)
    }
    Ok(paths)
}
