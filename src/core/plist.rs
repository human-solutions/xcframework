use super::platform::ApplePlatform;

pub struct InfoPlistBuilder {
    bundle_name: String,
    platform: ApplePlatform,
}

impl InfoPlistBuilder {
    pub fn new(bundle_name: &str, platform: ApplePlatform) -> Self {
        Self {
            bundle_name: bundle_name.into(),
            platform,
        }
    }

    pub fn bundle_name(mut self, bundle_name: &str) -> Self {
        self.bundle_name = bundle_name.to_string();
        self
    }

    pub fn platform(mut self, platform: ApplePlatform) -> Self {
        self.platform = platform;
        self
    }

    pub fn write(&self, path: &str) -> std::io::Result<()> {
        let template = TEAMPLATE
            .replace("{BUNDLE_NAME}", &self.bundle_name)
            .replace("{SUPPORTED_PLATFORM}", self.platform.platform_name())
            .replace("{PLATFORM_NAME}", self.platform.platform_display_name());
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
	<string>iphonesimulator13.0</string>
	<key>MinimumOSVersion</key>
	<string>13.0</string>
</dict>
</plist>
"###;
