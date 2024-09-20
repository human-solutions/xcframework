#![allow(non_snake_case)]

use super::Target;
use anyhow::{bail, Context, Result};
use camino::{Utf8Path, Utf8PathBuf};
use serde::Deserialize;
use std::str::FromStr;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LibType {
    StaticLib,
    CDyLib,
}

impl FromStr for LibType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "staticlib" => Ok(LibType::StaticLib),
            "cdylib" => Ok(LibType::CDyLib),
            _ => Err(format!("Unknown lib type: {}", s)),
        }
    }
}

impl LibType {
    pub fn file_ending(&self) -> &'static str {
        match self {
            LibType::StaticLib => "a",
            LibType::CDyLib => "dylib",
        }
    }
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct XCFrameworkConfiguration {
    /// The include directory containing the headers and the module.modulemap file
    pub include_dir: Utf8PathBuf,

    /// The library type (staticlib or cdylib)
    /// only necessary if the package lib target defines both
    pub lib_type: Option<LibType>,

    /// Whether to zip the resulting XCFramework
    #[serde(default = "zip_default")]
    pub zip: bool,

    /// Enable Cargo to compile the standard library itself as part of a crate graph compilation.
    #[serde(default)]
    pub build_std: bool,

    #[serde(default)]
    pub macOS: bool,

    #[serde(default = "Target::default_macos")]
    pub macOS_targets: Vec<Target>,

    #[serde(default)]
    pub simulators: bool,

    #[serde(default)]
    pub iOS: bool,

    #[serde(default = "Target::default_ios")]
    pub iOS_targets: Vec<Target>,

    #[serde(default = "Target::default_ios_sim")]
    pub iOS_simulator_targets: Vec<Target>,
}

pub fn zip_default() -> bool {
    true
}

impl XCFrameworkConfiguration {
    pub fn chosen_targets(&self) -> Vec<Target> {
        let mut all = vec![];
        if self.macOS {
            all.extend(self.macOS_targets.iter());
        }
        if self.iOS {
            all.extend(self.iOS_targets.iter());
            if self.simulators {
                all.extend(self.iOS_simulator_targets.iter());
            }
        }
        all
    }

    /// Parses the [package.metadata.xcframework] section of the Cargo.toml
    /// and updates the headers_directory to be relative to current working directory
    pub fn parse(metadata: &serde_json::Value, dir: &Utf8Path) -> Result<Self> {
        if let Some(xcfr) = metadata.get("xcframework") {
            Self::parse_xcframework(xcfr, dir)
                .context("Error in Cargo.toml section [package.metadata.xcframework]")
        } else {
            bail!("Missing [package.metadata.xcframework] section in Cargo.toml");
        }
    }

    fn parse_xcframework(xcfr: &serde_json::Value, dir: &Utf8Path) -> Result<Self> {
        let mut me = serde_json::from_value::<Self>(xcfr.clone())?;
        me.include_dir = dir.join(me.include_dir);
        me.validated()
    }

    fn validated(self) -> Result<Self> {
        if !self.include_dir.exists() {
            bail!("The include-dir '{}' does not exist", self.include_dir);
        }

        if !self.iOS && !self.macOS {
            bail!("Nothing to build. At least one the fields 'iOS' or 'macOS' must be set to true");
        }
        Ok(self)
    }
}
