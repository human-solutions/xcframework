use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Target {
    IosDevice,
    IosSimArm64,
    IosSimX86_64,
    MacosArm64,
    MacosX86_64,
}

impl<'de> Deserialize<'de> for Target {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "aarch64-apple-ios" | "iOS_Device" | "IosDevice" => Ok(Target::IosDevice),
            "aarch64-apple-ios-sim" | "iOS_aarch_Simulator" | "IosSimArm64" => {
                Ok(Target::IosSimArm64)
            }
            "x86_64-apple-ios" | "iOS_x86_Simulator" | "IosSimX86_64" => Ok(Target::IosSimX86_64),
            "aarch64-apple-darwin" | "macOS_aarch" | "MacosArm64" => Ok(Target::MacosArm64),
            "x86_64-apple-darwin" | "macOS_x86" | "MacosX86_64" => Ok(Target::MacosX86_64),
            _ => Err(serde::de::Error::unknown_variant(
                &s,
                &[
                    "aarch64-apple-ios",
                    "aarch64-apple-ios-sim",
                    "x86_64-apple-ios",
                    "aarch64-apple-darwin",
                    "x86_64-apple-darwin",
                ],
            )),
        }
    }
}

impl FromStr for Target {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "aarch64-apple-ios-sim" => Ok(Target::IosSimArm64),
            "x86_64-apple-ios" => Ok(Target::IosSimX86_64),
            "aarch64-apple-ios" => Ok(Target::IosDevice),
            "x86_64-apple-darwin" => Ok(Target::MacosX86_64),
            "aarch64-apple-darwin" => Ok(Target::MacosArm64),
            _ => Err(format!("Unknown target: {s}")),
        }
    }
}

impl Target {
    pub fn default_macos() -> Vec<Target> {
        vec![Target::MacosX86_64, Target::MacosArm64]
    }
    pub fn default_ios() -> Vec<Target> {
        vec![Target::IosDevice]
    }
    pub fn default_ios_sim() -> Vec<Target> {
        vec![Target::IosSimArm64, Target::IosSimX86_64]
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Target::IosDevice => "aarch64-apple-ios",
            Target::IosSimArm64 => "aarch64-apple-ios-sim",
            Target::IosSimX86_64 => "x86_64-apple-ios",
            Target::MacosArm64 => "aarch64-apple-darwin",
            Target::MacosX86_64 => "x86_64-apple-darwin",
        }
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
