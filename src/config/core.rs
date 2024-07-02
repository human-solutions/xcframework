use crate::core::{
    build::Target,
    platform::{ApplePlatform, Environment, EnvironmentWithoutCatalyst},
};

use super::{Architecture, Config, SupportedTargetPlatform};

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
