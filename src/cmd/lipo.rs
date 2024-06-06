use std::collections::HashMap;

use crate::conf::Configuration;
use crate::core::platform::{DarwinPlatform, Environment};
use anyhow::Result;
use camino::Utf8PathBuf;
use rustup_configurator::target::Triple;

pub fn assemble_libs(conf: &Configuration) -> Result<Vec<String>> {
    let libs_dir = conf.build_dir.join("libs");
    fs_err::create_dir_all(&libs_dir)?;

    let mut platform_lib_paths = HashMap::new();
    if conf.cargo_section.iOS {
        let lib_paths = lib_paths_for_targets(conf, &conf.cargo_section.iOS_targets)?;
        platform_lib_paths.insert(DarwinPlatform::IOS(Environment::Device), lib_paths);
    }
    if conf.cargo_section.simulators {
        let lib_paths = lib_paths_for_targets(conf, &conf.cargo_section.iOS_simulator_targets)?;
        platform_lib_paths.insert(DarwinPlatform::IOS(Environment::Simulator), lib_paths);
    }
    if conf.cargo_section.macOS {
        let lib_paths = lib_paths_for_targets(conf, &conf.cargo_section.macOS_targets)?;
        platform_lib_paths.insert(DarwinPlatform::MacOS, lib_paths);
    }

    let ending = conf.lib_type.file_ending();
    let name = &conf.lib_name.replace('-', "_");
    let output_lib_name = format!("lib{name}.{ending}");
    let output = crate::core::lipo_create_platform_libraries(
        &platform_lib_paths,
        &output_lib_name,
        &libs_dir,
    )?;

    Ok(output.values().into_iter().map(|p| p.to_string()).collect())
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
