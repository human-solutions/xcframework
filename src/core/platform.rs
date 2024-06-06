//! An XCFramework bundle, or artifact, is a binary package created by Xcode that includes the frameworks and libraries necessary to build for
//! multiple platforms (iOS, macOS, visionOS, tvOS, watchOS, and DriverKit), including Simulator builds.

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum DarwinPlatform {
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

impl DarwinPlatform {
    pub fn platform_display_name(&self) -> &'static str {
        match self {
            DarwinPlatform::MacOS => "macOS",
            DarwinPlatform::IOS(Environment::Device) => "iOS",
            DarwinPlatform::IOS(Environment::Simulator) => "iOS Simulator",
            DarwinPlatform::IOS(Environment::Catalyst) => "Mac Catalyst",
            DarwinPlatform::TvOS(EnvironmentWithoutCatalyst::Device) => "tvOS",
            DarwinPlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => "tvOS Simulator",
            DarwinPlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => "watchOS",
            DarwinPlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => "watchOS Simulator",
        }
    }

    pub fn platform_name(&self) -> &'static str {
        match self {
            DarwinPlatform::MacOS => "macosx",
            DarwinPlatform::IOS(Environment::Device) => "iphoneos",
            DarwinPlatform::IOS(Environment::Simulator) => "iphonesimulator",
            DarwinPlatform::IOS(Environment::Catalyst) => "maccatalyst",
            DarwinPlatform::TvOS(EnvironmentWithoutCatalyst::Device) => "appletvos",
            DarwinPlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => "appletvsimulator",
            DarwinPlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => "watchos",
            DarwinPlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => "watchsimulator",
        }
    }

    pub fn linker_platform_name(&self) -> &'static str {
        match self {
            DarwinPlatform::MacOS => "macos",
            DarwinPlatform::IOS(Environment::Device) => "ios",
            DarwinPlatform::IOS(Environment::Simulator) => "ios-simulator",
            DarwinPlatform::IOS(Environment::Catalyst) => "mac-catalyst",
            DarwinPlatform::TvOS(EnvironmentWithoutCatalyst::Device) => "tvos",
            DarwinPlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => "tvos-simulator",
            DarwinPlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => "watchos",
            DarwinPlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => "watchos-simulator",
        }
    }

    pub fn library_name_suffix(&self) -> &'static str {
        match self {
            DarwinPlatform::MacOS => "osx",
            DarwinPlatform::IOS(Environment::Device) => "ios",
            DarwinPlatform::IOS(Environment::Simulator) => "iossim",
            DarwinPlatform::IOS(Environment::Catalyst) => "osx",
            DarwinPlatform::TvOS(EnvironmentWithoutCatalyst::Device) => "tvos",
            DarwinPlatform::TvOS(EnvironmentWithoutCatalyst::Simulator) => "tvossim",
            DarwinPlatform::WatchOS(EnvironmentWithoutCatalyst::Device) => "watchos",
            DarwinPlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator) => "watchossim",
        }
    }
}
