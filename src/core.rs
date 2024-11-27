use std::collections::HashMap;

use anyhow::{Context, Ok};
use camino_fs::*;
use platform::ApplePlatform;
use xshell::{cmd, Shell};

pub mod platform;
pub mod plist;

/// The frameworks can be static or dynamic.
/// From rust perspective, it's crate type: cdylib or staticlib.
#[derive(Debug, Clone, Copy)]
pub enum CrateType {
    Cdylib,
    Staticlib,
}

/// Create a universal library for each platform using lipo.
pub fn lipo_create_platform_libraries(
    platform_lib_paths: &HashMap<ApplePlatform, Vec<Utf8PathBuf>>,
    output_lib_name: &str,
    output_dir: &Utf8PathBuf,
) -> anyhow::Result<HashMap<ApplePlatform, Utf8PathBuf>> {
    let sh = Shell::new()?;
    output_dir.mkdirs()?;

    let mut libs = HashMap::new();
    for (platform, paths) in platform_lib_paths.iter() {
        if paths.len() == 1 {
            // No need to lipo
            libs.insert(platform.clone(), paths[0].clone());
            continue;
        }
        let platform_dir = output_dir.join(format!("{:?}", platform));
        platform_dir.mkdirs()?;
        let output_path = platform_dir.join(output_lib_name);

        let mut cmd = cmd!(sh, "lipo -create");
        for path in paths {
            cmd = cmd.arg(path);
        }
        cmd = cmd.arg("-output").arg(&output_path);
        println!("🍭 Running lipo create for platform: {platform:?} ...");
        cmd.run()?;
        println!("✅ Run lipo create success, platform: {platform:?}, output:\n{output_path}");
        libs.insert(platform.clone(), output_path);
    }
    Ok(libs)
}

/// Reference: [article](https://developer.apple.com/documentation/xcode/creating-a-multi-platform-binary-framework-bundle#Determine-the-architectures-a-binary-supports)
///
/// Avoid using dynamic library files (.dylib files) for dynamic linking.
/// An XCFramework can include dynamic library files, but only macOS supports these libraries for dynamic linking.
/// Dynamic linking on iOS, watchOS, and tvOS requires the XCFramework to contain .framework bundles.
pub fn wrap_as_framework(
    platform: ApplePlatform,
    crate_type: &CrateType,
    lib_path: &Utf8PathBuf,
    header_paths: Vec<Utf8PathBuf>,
    module_path: Utf8PathBuf,
    bundle_name: &str,
    output_dir: &Utf8PathBuf,
) -> anyhow::Result<Utf8PathBuf> {
    const SUFFIX: &str = ".framework";

    println!("📦 Wrapping {:?} libraries as framework ...", platform);

    let sh = Shell::new()?;

    let output_path = output_dir
        .join(format!("{:?}", platform))
        .join(format!("{}{}", bundle_name, SUFFIX));
    output_path.mkdirs()?;

    let plist = plist::InfoPlistBuilder::new(bundle_name, platform);
    let plist_path = output_path.join("Info.plist");
    plist.write(plist_path.as_str())?;

    sh.cmd("plutil")
        .args(&[
            "-convert",
            "binary1",
            "-o",
            &format!("{}/Info.plist", output_path),
            plist_path.as_str(),
        ])
        .run()?;

    let to_binary = format!("{}/{}", &output_path, bundle_name);
    lib_path.cp(to_binary)?;

    if let CrateType::Cdylib = crate_type {
        sh.cmd("install_name_tool")
            .args([
                "-id",
                &format!("@rpath/{}.framework/{}", bundle_name, bundle_name),
                &format!("{}/{}", &output_path, bundle_name),
            ])
            .output()?;
    }

    output_path.join("Headers").mkdirs()?;
    output_path.join("Modules").mkdirs()?;

    for header_path in header_paths.iter() {
        let header_name = header_path.file_name().context("header path error")?;
        header_path.cp(output_path.join("Headers").join(header_name))?;
    }

    let module_dest = output_path.join("Modules").join("module.modulemap");
    module_path.cp(module_dest)?;

    println!(
        "✅ Wrapped artifacts as framework success, output:\n{}",
        output_path
    );
    Ok(output_path)
}

/// Create an XCFramework from the frameworks.
pub fn create_xcframework(
    framework_paths: Vec<Utf8PathBuf>,
    bundle_name: &str,
    output_dir: &Utf8PathBuf,
) -> anyhow::Result<Utf8PathBuf> {
    const SUFFIX: &str = ".xcframework";

    println!("🧰 Running create xcframework...");

    let sh = Shell::new()?;

    let xcframework_path = output_dir.join(format!("{}{}", bundle_name, SUFFIX));

    if xcframework_path.exists() {
        xcframework_path.rm()?;
    }

    let mut cmd = sh.cmd("xcrun").args(["xcodebuild", "-create-xcframework"]);
    for path in framework_paths.iter() {
        cmd = cmd.args(["-framework", path.as_str()]);
    }
    cmd = cmd.args(["-output", xcframework_path.as_str()]);

    cmd.run()?;
    println!("✅ Run create xcframework success, output:\n{xcframework_path}");

    Ok(xcframework_path)
}

/// Compress the XCFramework as a zip file.
pub fn compress_xcframework(
    xcframework_name: Option<String>,
    xcframework_path: &Utf8PathBuf,
    prefix: Option<String>,
    output_dir: &Utf8PathBuf,
) -> anyhow::Result<Utf8PathBuf> {
    println!("📦 Compressing XCFramework ...");

    let framework_name = xcframework_name
        .clone()
        .or_else(|| xcframework_path.file_name().map(Into::into))
        .context("Missing xcframework name")?;

    let zip_name = match &prefix {
        Some(prefix) => format!("{prefix}_{}.zip", framework_name),
        None => format!("{}.zip", framework_name),
    };
    let dest = output_dir.join(zip_name);
    let source = xcframework_path;
    zip_extensions::zip_create_from_directory(
        &dest.clone().into_std_path_buf(),
        &source.clone().into_std_path_buf(),
    )?;

    println!("✅ Compressed XCFramework success, output:\n{dest}");
    Ok(dest)
}
