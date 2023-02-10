#![allow(non_snake_case)]

use anyhow::{bail, Context, Result};
use camino::Utf8PathBuf;
use serde::Deserialize;
use std::str::FromStr;
use target_lexicon::{triple, OperatingSystem, Triple};

use crate::ext::TripleExt;

lazy_static::lazy_static! {
    static ref IOS_DEFAULT: Vec<Triple> = vec![triple!("aarch64-apple-ios")];
    static ref IOS_SIM_DEFAULT: Vec<Triple> =vec![triple!("aarch64-apple-ios-sim"), triple!("x86_64-apple-ios")];
    static ref MACOS_DEFAULT: Vec<Triple> = vec![triple!("x86_64-apple-darwin"), triple!("aarch64-apple-darwin")];
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum LibType {
    StaticLib,
    CDyLib,
}

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "kebab-case")]
pub struct XCFrameworkConfiguration {
    /// The headers include directory
    pub headers_directory: Utf8PathBuf,

    /// The library type (staticlib or cdylib)
    /// only necessary if the package lib target defines both
    pub lib_type: Option<LibType>,

    #[serde(default)]
    pub simulators: bool,

    #[serde(default)]
    pub macOS: bool,

    #[serde(default = "macOS_default")]
    pub macOS_targets: Vec<Triple>,

    #[serde(default)]
    pub iOS: bool,

    #[serde(default = "iOS_targets")]
    pub iOS_targets: Vec<Triple>,

    #[serde(default = "iOS_sim_targets")]
    pub iOS_simulator_targets: Vec<Triple>,
}

pub fn macOS_default() -> Vec<Triple> {
    MACOS_DEFAULT.clone()
}

fn iOS_targets() -> Vec<Triple> {
    IOS_DEFAULT.clone()
}

fn iOS_sim_targets() -> Vec<Triple> {
    IOS_SIM_DEFAULT.clone()
}

impl XCFrameworkConfiguration {
    pub fn parse(metadata: &serde_json::Value) -> Result<Self> {
        if let Some(xcfr) = metadata.get("xcframework") {
            Self::parse_xcframework(xcfr)
                .context("Error in Cargo.toml section [package.metadata.xcframework]")
        } else {
            bail!("Missing [package.metadata.xcframework] section in Cargo.toml");
        }
    }

    fn parse_xcframework(xcfr: &serde_json::Value) -> Result<Self> {
        serde_json::from_value::<Self>(xcfr.clone())?.validated()
    }

    fn validated(self) -> Result<Self> {
        if !self.headers_directory.exists() {
            bail!(
                "The headers-directory '{}' does not exist",
                self.headers_directory
            );
        }

        if !self.iOS && !self.macOS {
            bail!("Nothing to build. At least one the fields 'iOS' or 'macOS' must be set to true");
        }
        use target_lexicon::OperatingSystem::*;
        validate_triples(&self.macOS_targets, &Darwin, false).context("field 'macOS-targets'")?;
        validate_triples(&self.iOS_targets, &Ios, false).context("field 'iOS-targets'")?;
        validate_triples(&self.iOS_simulator_targets, &Ios, true)
            .context("field 'iOS-simulator-targets'")?;
        Ok(self)
    }
}

fn validate_triples(targets: &Vec<Triple>, os: &OperatingSystem, simulator: bool) -> Result<()> {
    for triple in targets {
        if triple.operating_system != *os {
            bail!("expected {os} not {} in {triple}", triple.architecture);
        }
        use target_lexicon::Vendor::Apple;
        if triple.vendor != Apple {
            bail!("expected {Apple} not {} in {triple}", triple.architecture,);
        }
        if simulator && !triple.is_apple_simulator() {
            bail!("expected a simulator architecture not {triple}");
        }
    }
    Ok(())
}
