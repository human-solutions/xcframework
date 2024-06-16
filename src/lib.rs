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
mod conf;
pub mod core;
pub mod ext;

use crate::conf::Configuration;
use anyhow::{Context, Result};
use camino::Utf8PathBuf;
use cmd::{cargo, lipo, rustup, xcodebuild, zip};
pub use conf::{XCFrameworkConfiguration, XcCli};
use ext::PathBufExt;

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

    let libs = lipo::assemble_libs(&conf).context("lipo: assembling libraries")?;

    let bundle_name = conf.module_name()?;
    let frameworks = libs
        .into_iter()
        .filter_map(|(platform, lib_path)| {
            let crate_type = conf.lib_type.clone().into();
            let include_dir = &conf.cargo_section.include_dir;
            let header_paths = get_header_paths(include_dir).ok()?;
            let module_paths = get_module_paths(include_dir).ok()?;
            let frameworks_dir = conf.target_dir.join("frameworks");
            std::fs::create_dir_all(&frameworks_dir).ok()?;

            core::wrap_as_framework(
                platform,
                crate_type,
                lib_path,
                header_paths,
                module_paths,
                &bundle_name,
                frameworks_dir,
            )
            .ok()
        })
        .collect::<Vec<_>>();

    xcodebuild::assemble(&conf, frameworks).context("xcodebuild - assemble libraries")?;
    let module_name = conf.module_name()?;

    let (path, is_zipped) = if conf.cargo_section.zip {
        (zip::xcframework(&conf)?, true)
    } else {
        let from = conf.build_dir.join(format!("{module_name}.xcframework"));
        let to = conf.target_dir.join(format!("{module_name}.xcframework"));
        fs_err::rename(from, &to)?;
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

#[cfg(test)]
#[test]
fn update_readme() {
    markdown_includes::update("src/readme.tpl.md", "README.md").unwrap();
}
