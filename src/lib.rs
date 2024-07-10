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

pub mod config;
pub mod core;
pub mod ext;

pub use cli::XcframeworkOp;
mod cli;

#[cfg(test)]
#[test]
fn update_readme() {
    markdown_includes::update("src/readme.tpl.md", "README.md").unwrap();
}
