use super::platform::ApplePlatform;

pub struct InfoPlistBuilder {
    bundle_name: String,
    platform: ApplePlatform,
    sdk_version: String,
    min_os_version: String,
}

impl InfoPlistBuilder {
    pub fn new(
        bundle_name: &str,
        platform: ApplePlatform,
        sdk_version: String,
        min_os_version: String,
    ) -> Self {
        Self {
            bundle_name: bundle_name.into(),
            platform,
            sdk_version,
            min_os_version,
        }
    }

    pub fn write(&self, path: &str) -> std::io::Result<()> {
        let template = TEAMPLATE
            .replace("{BUNDLE_NAME}", &self.bundle_name)
            .replace("{SUPPORTED_PLATFORM}", self.platform.platform_name())
            .replace("{PLATFORM_NAME}", self.platform.platform_name())
            .replace("{SDK_VERSION}", &self.sdk_version)
            .replace("{MIN_OS_VERSION}", &self.min_os_version);
        std::fs::write(path, template)
    }
}

const TEAMPLATE: &str = r###"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleExecutable</key>
	<string>{BUNDLE_NAME}</string>
	<key>CFBundleIdentifier</key>
	<string>xcframework.cargo.{BUNDLE_NAME}</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>{BUNDLE_NAME}</string>
	<key>CFBundlePackageType</key>
	<string>APPL</string>
	<key>CFBundleShortVersionString</key>
	<string>1.0</string>
	<key>CFBundleSupportedPlatforms</key>
	<array>
		<string>{SUPPORTED_PLATFORM}</string>
	</array>
	<key>CFBundleVersion</key>
	<string>1</string>
	<key>DTPlatformName</key>
	<string>{PLATFORM_NAME}</string>
	<key>DTSDKName</key>
	<string>{PLATFORM_NAME}{SDK_VERSION}</string>
	<key>MinimumOSVersion</key>
	<string>{MIN_OS_VERSION}</string>
</dict>
</plist>
"###;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::platform::{Environment, EnvironmentWithoutCatalyst};

    #[test]
    fn plist_platform_values_match_sdk_identifiers() {
        let platforms = vec![
            (ApplePlatform::MacOS, "17.0", "10.12"),
            (ApplePlatform::IOS(Environment::Device), "18.0", "15.0"),
            (ApplePlatform::IOS(Environment::Simulator), "18.0", "15.0"),
            (ApplePlatform::IOS(Environment::Catalyst), "18.0", "15.0"),
            (
                ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Device),
                "18.0",
                "16.0",
            ),
            (
                ApplePlatform::TvOS(EnvironmentWithoutCatalyst::Simulator),
                "18.0",
                "16.0",
            ),
            (
                ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Device),
                "11.0",
                "9.0",
            ),
            (
                ApplePlatform::WatchOS(EnvironmentWithoutCatalyst::Simulator),
                "11.0",
                "9.0",
            ),
        ];

        for (platform, sdk_version, min_os_version) in platforms {
            let expected_name = platform.platform_name();
            let dir = tempfile::tempdir().unwrap();
            let path = dir.path().join("Info.plist");
            let path_str = path.to_str().unwrap();

            InfoPlistBuilder::new(
                "TestBundle",
                platform,
                sdk_version.to_string(),
                min_os_version.to_string(),
            )
            .write(path_str)
            .unwrap();

            let contents = std::fs::read_to_string(path_str).unwrap();

            // DTPlatformName should be the SDK identifier
            let dt_platform = extract_plist_value(&contents, "DTPlatformName");
            assert_eq!(
                dt_platform, expected_name,
                "DTPlatformName mismatch for {expected_name}"
            );

            // DTSDKName should be platform name + SDK version
            let dt_sdk = extract_plist_value(&contents, "DTSDKName");
            let expected_sdk = format!("{expected_name}{sdk_version}");
            assert_eq!(
                dt_sdk, expected_sdk,
                "DTSDKName mismatch for {expected_name}"
            );

            // MinimumOSVersion should match the provided value
            let min_os = extract_plist_value(&contents, "MinimumOSVersion");
            assert_eq!(
                min_os, min_os_version,
                "MinimumOSVersion mismatch for {expected_name}"
            );
        }
    }

    #[test]
    fn plist_with_empty_sdk_version() {
        let platform = ApplePlatform::MacOS;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("Info.plist");
        let path_str = path.to_str().unwrap();

        InfoPlistBuilder::new("TestBundle", platform, String::new(), "10.0".to_string())
            .write(path_str)
            .unwrap();

        let contents = std::fs::read_to_string(path_str).unwrap();

        // DTSDKName should be just the platform name with no trailing version
        let dt_sdk = extract_plist_value(&contents, "DTSDKName");
        assert_eq!(
            dt_sdk, "macosx",
            "DTSDKName should be just platform name when sdk_version is empty"
        );

        // MinimumOSVersion should still be set correctly
        let min_os = extract_plist_value(&contents, "MinimumOSVersion");
        assert_eq!(min_os, "10.0", "MinimumOSVersion should still be set");
    }

    fn extract_plist_value(plist: &str, key: &str) -> String {
        let key_tag = format!("<key>{key}</key>");
        let after_key = plist.split(&key_tag).nth(1).unwrap();
        let value = after_key
            .split("<string>")
            .nth(1)
            .unwrap()
            .split("</string>")
            .next()
            .unwrap();
        value.to_string()
    }
}
