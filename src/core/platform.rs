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
