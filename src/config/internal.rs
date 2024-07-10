use std::process::exit;

use anyhow::{bail, Result};
use dialoguer::Confirm;

use crate::core::{
    build::Target,
    modulemap,
    platform::{ApplePlatform, Environment, EnvironmentWithoutCatalyst},
};

use super::{Architecture, Config, LibType, SupportedTargetPlatform};

impl LibType {
    pub fn ext(&self) -> &str {
        match self {
            LibType::Staticlib => "a",
            LibType::Cdylib => "dylib",
        }
    }
}

impl Config {
    pub fn targets(&self) -> Vec<Target> {
        let mut targets = Vec::new();
        for (platform, config) in self.platforms.iter() {
            match config {
                super::TargetPlatformConfigVariant::Preset(enable) => {
                    if *enable {
                        targets.extend(platform.preset_targets())
                    }
                }
                super::TargetPlatformConfigVariant::Custom(config) => {
                    if config.enable {
                        targets.extend(platform.to_targets(config.simulator, &config.archs))
                    }
                }
            }
        }
        targets
    }

    pub fn module_name(&self) -> Result<String> {
        if let Some(name) = self.module_name.clone() {
            Ok(name)
        } else {
            let mm_path = self.include_dir.join("module.modulemap");
            let name = modulemap::get_module_name(&mm_path)?;
            Ok(name)
        }
    }

    pub fn check_rustup(&self) -> Result<()> {
        let targets = rustup_configurator::target::list()?;

        let mut to_install = vec![];
        for needed_target in self.targets() {
            let Some(target) = targets.iter().find(|t| t.triple == *needed_target.triple) else {
                bail!("Target {} is not supported by rustup", needed_target.triple)
            };

            if !target.installed {
                to_install.push(target.triple.clone());
            }
        }
        if !to_install.is_empty() {
            let do_install = Confirm::new()
                .with_prompt(format!(
                    "The targets {} are missing, do you want to install them?",
                    to_install
                        .iter()
                        .map(|t| t.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                ))
                .interact()?;
            if do_install {
                rustup_configurator::target::install(&to_install)?
            } else {
                exit(1);
            }
        }
        Ok(())
    }
}

impl SupportedTargetPlatform {
    fn preset_targets(&self) -> Vec<Target> {
        match self {
            SupportedTargetPlatform::IOS => vec![
                Target {
                    triple: "aarch64-apple-ios".to_string(),
                    platform: ApplePlatform::IOS(Environment::Device),
                },
                Target {
                    triple: "aarch64-apple-ios-sim".to_string(),
                    platform: ApplePlatform::IOS(Environment::Simulator),
                },
                Target {
                    triple: "x86_64-apple-ios".to_string(),
                    platform: ApplePlatform::IOS(Environment::Simulator),
                },
            ],
            SupportedTargetPlatform::MacOS => vec![
                Target {
                    triple: "x86_64-apple-darwin".to_string(),
                    platform: ApplePlatform::MacOS,
                },
                Target {
                    triple: "aarch64-apple-darwin".to_string(),
                    platform: ApplePlatform::MacOS,
                },
            ],
            SupportedTargetPlatform::TvOS => vec![
                Target {
                    triple: "aarch64-apple-tvos".to_string(),
                    platform: ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device),
                },
                Target {
                    triple: "aarch64-apple-tvos-sim".to_string(),
                    platform: ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator),
                },
                Target {
                    triple: "x86_64-apple-tvos".to_string(),
                    platform: ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator),
                },
            ],
            SupportedTargetPlatform::WatchOS => vec![
                Target {
                    triple: "aarch64-apple-watchos".to_string(),
                    platform: ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device),
                },
                Target {
                    triple: "aarch64-apple-watchos-sim".to_string(),
                    platform: ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator),
                },
                Target {
                    triple: "x86_64-apple-watchos-sim".to_string(),
                    platform: ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator),
                },
            ],
        }
    }

    // Reference: https://doc.rust-lang.org/rustc/platform-support.html - Apple platforms(*-apple-*)
    fn to_targets(&self, enable_sim: bool, archs: &[Architecture]) -> Vec<Target> {
        match self {
            // `-apple-ios`
            SupportedTargetPlatform::IOS => {
                let mut targets = Vec::new();
                if archs.contains(&Architecture::Aarch64) {
                    targets.push(Target {
                        triple: "aarch64-apple-ios".to_string(),
                        platform: ApplePlatform::IOS(Environment::Device),
                    });
                    if enable_sim {
                        targets.push(Target {
                            triple: "aarch64-apple-ios-sim".to_string(),
                            platform: ApplePlatform::IOS(Environment::Simulator),
                        });
                    }
                }
                if archs.contains(&Architecture::X86_64) {
                    targets.push(Target {
                        triple: "x86_64-apple-ios".to_string(),
                        platform: ApplePlatform::IOS(Environment::Simulator),
                    });
                }
                if archs.contains(&Architecture::Arm64e) {
                    targets.push(Target {
                        triple: "arm64e-apple-ios".to_string(),
                        platform: ApplePlatform::IOS(Environment::Device),
                    });
                }
                targets
            }
            // `-apple-darwin`
            SupportedTargetPlatform::MacOS => {
                let mut targets = Vec::new();
                if archs.contains(&Architecture::X86_64) {
                    targets.push(Target {
                        triple: "x86_64-apple-darwin".to_string(),
                        platform: ApplePlatform::MacOS,
                    });
                }
                if archs.contains(&Architecture::Aarch64) {
                    targets.push(Target {
                        triple: "aarch64-apple-darwin".to_string(),
                        platform: ApplePlatform::MacOS,
                    });
                }
                if archs.contains(&Architecture::Arm64e) {
                    targets.push(Target {
                        triple: "arm64e-apple-darwin".to_string(),
                        platform: ApplePlatform::MacOS,
                    });
                }
                targets
            }
            // `-apple-tvos`
            SupportedTargetPlatform::TvOS => {
                let mut targets = Vec::new();
                if archs.contains(&Architecture::Aarch64) {
                    targets.push(Target {
                        triple: "aarch64-apple-tvos".to_string(),
                        platform: ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device),
                    });
                    if enable_sim {
                        targets.push(Target {
                            triple: "aarch64-apple-tvos-sim".to_string(),
                            platform: ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator),
                        });
                    }
                }
                if archs.contains(&Architecture::X86_64) {
                    targets.push(Target {
                        triple: "x86_64-apple-tvos".to_string(),
                        platform: ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator),
                    });
                }
                targets
            }
            // `-apple-watchos`
            SupportedTargetPlatform::WatchOS => {
                let mut targets = Vec::new();
                if archs.contains(&Architecture::Aarch64) {
                    targets.push(Target {
                        triple: "aarch64-apple-watchos".to_string(),
                        platform: ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device),
                    });
                    if enable_sim {
                        targets.push(Target {
                            triple: "aarch64-apple-watchos-sim".to_string(),
                            platform: ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator),
                        });
                    }
                }
                if archs.contains(&Architecture::X86_64) {
                    targets.push(Target {
                        triple: "x86_64-apple-watchos".to_string(),
                        platform: ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator),
                    });
                }
                targets
            }
        }
    }
}
