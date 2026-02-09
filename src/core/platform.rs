//! An XCFramework bundle, or artifact, is a binary package created by Xcode that includes the frameworks and libraries necessary to build for
//! multiple platforms (iOS, macOS, visionOS, tvOS, watchOS, and DriverKit), including Simulator builds.

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum ApplePlatform {
    MacOS,
    IOS(Environment),
    TvOS(EnvironmentWithoutCatalyst),
    WatchOS(EnvironmentWithoutCatalyst),
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum Environment {
    Device,
    Simulator,
    Catalyst,
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum EnvironmentWithoutCatalyst {
    Device,
    Simulator,
}

impl ApplePlatform {
    pub fn platform_display_name(&self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "macOS",
            ApplePlatform::IOS(Environment::Device) => "iOS",
            ApplePlatform::IOS(Environment::Simulator) => "iOS Simulator",
            ApplePlatform::IOS(Environment::Catalyst) => "Mac Catalyst",
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device) => "tvOS",
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => "tvOS Simulator",
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => "watchOS",
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => "watchOS Simulator",
        }
    }

    pub fn platform_name(&self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "macosx",
            ApplePlatform::IOS(Environment::Device) => "iphoneos",
            ApplePlatform::IOS(Environment::Simulator) => "iphonesimulator",
            ApplePlatform::IOS(Environment::Catalyst) => "maccatalyst",
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device) => "appletvos",
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => "appletvsimulator",
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => "watchos",
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => "watchsimulator",
        }
    }

    pub fn linker_platform_name(&self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "macos",
            ApplePlatform::IOS(Environment::Device) => "ios",
            ApplePlatform::IOS(Environment::Simulator) => "ios-simulator",
            ApplePlatform::IOS(Environment::Catalyst) => "mac-catalyst",
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device) => "tvos",
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => "tvos-simulator",
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => "watchos",
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => "watchos-simulator",
        }
    }

    pub fn library_name_suffix(&self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "osx",
            ApplePlatform::IOS(Environment::Device) => "ios",
            ApplePlatform::IOS(Environment::Simulator) => "iossim",
            ApplePlatform::IOS(Environment::Catalyst) => "osx",
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device) => "tvos",
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => "tvossim",
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => "watchos",
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => "watchossim",
        }
    }

    /// Returns the environment variable name for the deployment target of this platform.
    pub fn deployment_target_env_var(&self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "MACOSX_DEPLOYMENT_TARGET",
            ApplePlatform::IOS(_) => "IPHONEOS_DEPLOYMENT_TARGET",
            ApplePlatform::TvOS(_) => "TVOS_DEPLOYMENT_TARGET",
            ApplePlatform::WatchOS(_) => "WATCHOS_DEPLOYMENT_TARGET",
        }
    }

    /// Returns the default minimum deployment target for this platform,
    /// matching the Rust compiler's defaults.
    pub fn default_deployment_target(&self) -> &'static str {
        match self {
            ApplePlatform::MacOS => "10.12",
            ApplePlatform::IOS(_) => "10.0",
            ApplePlatform::TvOS(_) => "10.0",
            ApplePlatform::WatchOS(_) => "5.0",
        }
    }

    // Reference: https://doc.rust-lang.org/rustc/platform-support.html
    pub fn rustup_targets(&self) -> Vec<&str> {
        match self {
            ApplePlatform::MacOS => vec!["x86_64-apple-darwin", "aarch64-apple-darwin"],
            ApplePlatform::IOS(Environment::Device) => vec!["aarch64-apple-ios"],
            ApplePlatform::IOS(Environment::Simulator) => {
                vec!["x86_64-apple-ios", "aarch64-apple-ios-sim"]
            }
            ApplePlatform::IOS(Environment::Catalyst) => {
                vec!["x86_64-apple-ios-macabi", "aarch64-apple-ios-macabi"]
            }
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device) => vec!["aarch64-apple-tvos"],
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => {
                vec!["x86_64-apple-tvos", "aarch64-apple-tvos-sim"]
            }
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => {
                vec!["armv7k-apple-watchos", "arm64_32-apple-watchos"]
            }
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => {
                vec!["x86_64-apple-watchos", "aarch64-apple-tvos-sim"]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deployment_target_env_vars() {
        assert_eq!(
            ApplePlatform::MacOS.deployment_target_env_var(),
            "MACOSX_DEPLOYMENT_TARGET"
        );
        assert_eq!(
            ApplePlatform::IOS(Environment::Device).deployment_target_env_var(),
            "IPHONEOS_DEPLOYMENT_TARGET"
        );
        assert_eq!(
            ApplePlatform::IOS(Environment::Simulator).deployment_target_env_var(),
            "IPHONEOS_DEPLOYMENT_TARGET"
        );
        assert_eq!(
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device).deployment_target_env_var(),
            "TVOS_DEPLOYMENT_TARGET"
        );
        assert_eq!(
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device).deployment_target_env_var(),
            "WATCHOS_DEPLOYMENT_TARGET"
        );
    }

    #[test]
    fn default_deployment_targets() {
        assert_eq!(ApplePlatform::MacOS.default_deployment_target(), "10.12");
        assert_eq!(
            ApplePlatform::IOS(Environment::Device).default_deployment_target(),
            "10.0"
        );
        assert_eq!(
            ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator).default_deployment_target(),
            "10.0"
        );
        assert_eq!(
            ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device).default_deployment_target(),
            "5.0"
        );
    }
}
