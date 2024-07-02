//!
//! > <span style="color:darkorange">**⚠️ WARNING**</span>
//! >
//! > This is work in progress and is not ready for use
//!
//! A Cargo plugin and library for building Apple XCFrameworks from Rust libraries
//!
//! <br/>
//!
//! - [Features](#features)
//! - [Getting started](#getting-started)
//! - [Configuration](#configuration)
//!
//! <br/>
//!
//! # Features
//!
//! - Easily create Apple XCFrameworks from Rust libraries
//! - Integrates into the cargo build process. Run it with `cargo xcframework` with parameters that are almost the same as for `cargo build`
//! - Configuration in `Cargo.toml` section `[package.metadata.xcframework]`
//! - Currently supports building for iOS, macOS and simulators. If there is interest, watchOS and tvOS can be added as well.
//!
//! <br/>
//!
//! # Getting started
//!
//! Install:
//!
//! > `cargo install xcframework`
//!
//! If you for any reason needs the bleeding-edge super fresh version:
//!
//! > `cargo install --git https://github.com/human-solutions/xcframework`
//!
//! Help:
//!
//! > `cargo xcframework --help`
//!
//! For setting up your project, have a look at the [examples](https://github.com/akesson/cargo-xcframework/tree/main/examples)
//!
//! <br/>
//!
//! # Configuration
//!
//! The built XCFramework is named after the top-level module name declared in the `module.modulemap` file in the `include-dir` directory.
//!
//! A typical such file looks like this:
//!
//! ```cpp
//! // The XCFramework will be named 'MyModuleName.xcframework'
//! module MyModuleName {
//!     // a header file in the same directory as the modulemap
//!     header "mylib.h"
//!     export *
//! }
//! ```
//!
//! Cargo.toml parameters in section `[package.metadata.xcframework]`.
//!
//! ```toml
//! # Directory where the `module.modulemap` file and the headers are located.
//! #
//! # Note that the modulemap needs to be present in the directory because the
//! # module name declared in it is used for the framework name.
//! include-dir = "my-bin-name"
//!
//! # The library type. Can be staticlib or cdylib
//! #
//! # Optional. This is only necessary if both library types are configured in the
//! # [lib] sections `crate-type` parameter. Overridden by the command line parameter `--lib-type`.
//! lib-type = "staticlib"
//!
//! # Whether to zip the resulting XCFramework
//! #
//! # Optional. Defaults to true.
//! zip = true
//!
//! # Enable Cargo to compile the standard library itself as part of a crate graph compilation.
//! # If enabled either run with `cargo +nightly xcframework`, set the default toolchain to nightly
//! # or set run `rustup override set nightly` in the project directory.
//! #
//! # Optional. Defaults to false. Requires nightly. Only useful for staticlib's, ignored for cdylibs.
//! build-std = false
//!
//! # Whether to build for macOS
//! #
//! # Optional. Defaults to false.
//! macOS = false
//!
//! # The macOS target triples
//! #
//! # Optional. Defaults to ["x86_64-apple-darwin", "aarch64-apple-darwin"].
//! macOS-targets = ["x86_64-apple-darwin", "aarch64-apple-darwin"]
//!
//! # Whether to build the simulator targets. Not used when building for macOS.
//! #
//! # Optional. Defaults to false
//! simulators = false
//!
//! # Whether to build for iOS
//! #
//! # Optional. Defaults to false.
//! iOS = false
//!
//! # The iOS target triples
//! #
//! # Optional. Defaults to ["aarch64-apple-ios"].
//! iOS-targets = ["aarch64-apple-ios"]
//!
//!
//! # The iOS simulator target triples. Only used if `simulators` and `iOS` are true.
//! #
//! # Optional. Defaults to ["aarch64-apple-ios-sim", "x86_64-apple-ios"]
//! iOS-simulator-targets = ["aarch64-apple-ios-sim", "x86_64-apple-ios"]
//!
//! # If there is interest, watchOS and tvOS can be added as well.
//! ```
//!
//! The iOS and macOS versions targeted can be set with the environment variables:
//! `MACOSX_DEPLOYMENT_TARGET` and `IPHONEOS_DEPLOYMENT_TARGET`. See [apple_base.rs](https://github.com/rust-lang/rust/blob/master/compiler/rustc_target/src/spec/apple_base.rs) for the default values.
//!
mod cmd;

pub mod config;
pub mod core;
pub mod ext;
pub mod ops;

#[cfg(feature = "cli")]
mod cli; // TODO: deprecate or migrate to focus on parsing cli arguments to config;

use core::platform::{ApplePlatform, Environment};
use std::collections::HashMap;

use crate::cli::Configuration;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
pub use cli::{XCFrameworkConfiguration, XcCli};
use cmd::{cargo, rustup};
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
    let metadata = cargo_metadata::MetadataCommand::new()
        .manifest_path(&cli.clap_cargo.manifest_path()?)
        .exec()
        .context("cargo metadata")?;
    let package = metadata.root_package().context("no package")?;
    if let Ok(config) = config::load_package_config(&cli, package) {
        let res = ops::cargo::build(&cli, config, &metadata)?;
        return Ok(res);
    }

    // unreachable!("will remove");

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
        cli::LibType::StaticLib => &config::LibType::Staticlib,
        cli::LibType::CDyLib => &config::LibType::Cdylib,
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
                &header_paths,
                &module_paths,
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

#[cfg(test)]
#[test]
fn update_readme() {
    markdown_includes::update("src/readme.tpl.md", "README.md").unwrap();
}
