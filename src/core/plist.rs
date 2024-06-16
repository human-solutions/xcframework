use super::platform::DarwinPlatform;

pub struct InfoPlistBuilder {
    bundle_name: String,
    platform: DarwinPlatform,
}

impl InfoPlistBuilder {
    pub fn new(bundle_name: &str, platform: DarwinPlatform) -> Self {
        Self {
            bundle_name: bundle_name.into(),
            platform,
        }
    }

    pub fn bundle_name(mut self, bundle_name: &str) -> Self {
        self.bundle_name = bundle_name.to_string();
        self
    }

    pub fn platform(mut self, platform: DarwinPlatform) -> Self {
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

// TODO: simpliy the template
const TEAMPLATE: &str = r###"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>BuildMachineOSBuild</key>
	<string>23F79</string>
	<key>CFBundleDevelopmentRegion</key>
	<string>en</string>
	<key>CFBundleExecutable</key>
	<string>{BUNDLE_NAME}</string>
  <key>CFHeadersDirectory</key>
  <string>Headers</string>
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
	<key>DTCompiler</key>
	<string>com.apple.compilers.llvm.clang.1_0</string>
	<key>DTPlatformBuild</key>
	<string>21F77</string>
	<key>DTPlatformName</key>
	<string>{PLATFORM_NAME}</string>
	<key>DTPlatformVersion</key>
	<string>17.5</string>
	<key>DTSDKBuild</key>
	<string>21F77</string>
	<key>DTSDKName</key>
	<string>iphonesimulator17.5</string>
	<key>DTXcode</key>
	<string>1540</string>
	<key>DTXcodeBuild</key>
	<string>15F31d</string>
	<key>LSRequiresIPhoneOS</key>
	<true/>
	<key>MinimumOSVersion</key>
	<string>13.0</string>
	<key>NSHumanReadableCopyright</key>
	<string>Copyright Â©. All rights reserved.</string>
	<key>Test</key>
	<string>Value</string>
	<key>UIDeviceFamily</key>
	<array>
		<integer>1</integer>
		<integer>2</integer>
	</array>
</dict>
</plist>
"###;
