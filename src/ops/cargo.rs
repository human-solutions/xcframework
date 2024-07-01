use std::collections::HashMap;

use anyhow::{Context, Result};
use cargo_metadata::Metadata;

use crate::{
    config::{Config, LibType},
    core::build::lib_path_for_target,
    Produced, XcCli,
};

pub fn build_inner(cli: &XcCli, config: &Config) -> Result<()> {
    let mut cmd = cli.clap_cargo.build_cmd();

    if let Some(target_dir) = &cli.target_dir {
        cmd.args(["--target-dir", target_dir.as_str()]);
    }

    for target in config.targets() {
        cmd.args(["--target", target.triple.as_str()]);
    }

    cmd.status()?;

    Ok(())
}

// TODO: fix all testing
pub fn build(cli: &XcCli, config: Config, metadata: &Metadata) -> Result<Produced> {
    let root_package = metadata.root_package().context("Missing package")?;
    let target_dir = &metadata.target_directory;

    println!(
        "Building XCFramework for package: {:#?}",
        root_package.targets
    );

    build_inner(cli, &config)?;

    let pkg = &metadata.root_package().context("Missing package")?.name;
    let profile = "release";

    let bundle_name = config.name.as_ref().unwrap_or(pkg);

    let targets = config.targets();
    let lib_type = &config.lib_type;
    let sequentially = false;
    let libname = root_package
        .targets
        .iter()
        .find(|t| match lib_type {
            LibType::Cdylib => t.is_cdylib(),
            LibType::Staticlib => t.is_staticlib(),
        })
        .context("lib not found")?
        .name
        .as_str();

    let libs_dir = metadata.target_directory.join("libs");
    std::fs::create_dir_all(&libs_dir)?;

    let libname = format!("lib{libname}.{}", lib_type.ext());
    let mut platform_lib_paths = HashMap::new();
    for target in targets {
        let path = lib_path_for_target(&target_dir, target.triple.as_str(), profile, &libname);
        let paths = platform_lib_paths
            .entry(target.platform.clone())
            .or_insert_with(Vec::new);
        paths.push(path);
    }

    let libs =
        crate::core::lipo_create_platform_libraries(&platform_lib_paths, bundle_name, &libs_dir)?;

    // TODO: resolve absolute path
    let header_paths = config.header_paths;
    let module_paths = config.module_paths;
    let frameworks_dir = metadata.target_directory.join("frameworks");
    std::fs::create_dir_all(&frameworks_dir)?;

    let framework_paths = libs
        .into_iter()
        .map(|(platform, lib_path)| {
            std::fs::create_dir_all(&frameworks_dir)?;

            crate::core::wrap_as_framework(
                platform,
                lib_type,
                &lib_path,
                &header_paths,
                &module_paths,
                bundle_name,
                &frameworks_dir,
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let output_dir = metadata.target_directory.join("xcframeworks");
    let xcframework_path =
        crate::core::create_xcframework(framework_paths, bundle_name, &output_dir)?;

    println!("✅ Created XCFramework at {:?}", xcframework_path);

    let is_zipped = false;
    let path = xcframework_path;

    Ok(Produced {
        module_name: bundle_name.to_string(),
        path,
        is_zipped,
    })
}
