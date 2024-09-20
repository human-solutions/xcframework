use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum Target {
    iOS_Device,
    iOS_aarch_Simulator,
    iOS_x86_Simulator,
    macOS_aarch,
    macOS_x86,
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aarch64-apple-ios-sim" => Ok(Target::iOS_aarch_Simulator),
            "x86_64-apple-ios" => Ok(Target::iOS_x86_Simulator),
            "aarch64-apple-ios" => Ok(Target::iOS_Device),
            "x86_64-apple-darwin" => Ok(Target::macOS_x86),
            "aarch64-apple-darwin" => Ok(Target::macOS_aarch),
            _ => Err(format!("Unknown target: {s}")),
        }
    }
}

impl Target {
    pub fn default_macos() -> Vec<Target> {
        vec![Target::macOS_x86, Target::macOS_aarch]
    }
    pub fn default_ios() -> Vec<Target> {
        vec![Target::iOS_Device]
    }
    pub fn default_ios_sim() -> Vec<Target> {
        vec![Target::iOS_aarch_Simulator, Target::iOS_x86_Simulator]
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Target::iOS_Device => "aarch64-apple-ios",
            Target::iOS_aarch_Simulator => "aarch64-apple-ios-sim",
            Target::iOS_x86_Simulator => "x86_64-apple-ios",
            Target::macOS_aarch => "aarch64-apple-darwin",
            Target::macOS_x86 => "x86_64-apple-darwin",
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
