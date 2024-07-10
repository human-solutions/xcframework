use std::collections::HashMap;

use anyhow::{Context, Ok};
use camino::Utf8PathBuf;
use fs_err as fs;
use platform::ApplePlatform;
use xshell::{cmd, Shell};

use crate::config::LibType;

pub mod build;
pub mod modulemap;
pub mod platform;
pub mod plist;

/// Create a universal library for each platform using lipo.
pub fn lipo_create_platform_libraries(
    platform_lib_paths: &HashMap<ApplePlatform, Vec<Utf8PathBuf>>,
    output_lib_name: &str,
    output_dir: &Utf8PathBuf,
) -> anyhow::Result<HashMap<ApplePlatform, Utf8PathBuf>> {
    let sh = Shell::new()?;
    fs::create_dir_all(output_dir)?;

    let mut libs = HashMap::new();
    for (platform, paths) in platform_lib_paths.iter() {
        if paths.len() == 1 {
            // No need to lipo
            libs.insert(platform.clone(), paths[0].clone());
            continue;
        }
        let platform_dir = output_dir.join(format!("{:?}", platform));
        fs::create_dir_all(&platform_dir)?;
        let output_path = platform_dir.join(output_lib_name);
        if output_path.exists() {
            fs::remove_file(&output_path)?;
        }

        let mut cmd = cmd!(sh, "lipo -create");
        for path in paths {
            cmd = cmd.arg(path);
        }
        cmd = cmd.arg("-output").arg(&output_path);
        println!("🍭 Running lipo create for platform: {platform:?} ...");
        cmd.run()?;
        println!("✅ Run lipo create success, platform: {platform:?}, output:\n{output_path:?}");
        libs.insert(platform.clone(), output_path);
    }
    Ok(libs)
}

/// Reference: https://developer.apple.com/documentation/xcode/creating-a-multi-platform-binary-framework-bundle#Determine-the-architectures-a-binary-supports
///
/// Avoid using dynamic library files (.dylib files) for dynamic linking.
/// An XCFramework can include dynamic library files, but only macOS supports these libraries for dynamic linking.
/// Dynamic linking on iOS, watchOS, and tvOS requires the XCFramework to contain .framework bundles.
pub fn wrap_as_framework(
    platform: ApplePlatform,
    crate_type: &LibType,
    lib_path: &Utf8PathBuf,
    header_paths: &[Utf8PathBuf],
    module_paths: &[Utf8PathBuf],
    bundle_name: &str,
    output_dir: &Utf8PathBuf,
) -> anyhow::Result<Utf8PathBuf> {
    const SUFFIX: &str = ".framework";

    println!("📦 Wrapping {:?} libraries as framework ...", platform);

    let sh = Shell::new()?;

    let output_path = output_dir
        .join(format!("{:?}", platform))
        .join(format!("{}{}", bundle_name, SUFFIX));
    fs::create_dir_all(&output_path)?;

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

    let to_binary = output_path.join(bundle_name);
    if to_binary.exists() {
        fs::remove_file(&to_binary)?;
    }
    fs::copy(lib_path.as_path(), to_binary)?;

    if let LibType::Cdylib = crate_type {
        sh.cmd("install_name_tool")
            .args([
                "-id",
                &format!("@rpath/{}.framework/{}", bundle_name, bundle_name),
                &format!("{}/{}", &output_path, bundle_name),
            ])
            .output()?;
    }

    fs::create_dir_all(format!("{}/Headers", &output_path))?;
    fs::create_dir_all(format!("{}/Modules", &output_path))?;

    for header_path in header_paths.iter() {
        let header_name = header_path.file_name().context("header path error")?;
        fs::copy(
            header_path,
            format!("{}/Headers/{}", output_path, header_name),
        )?;
    }

    for module_path in module_paths.iter() {
        let module_name = module_path.file_name().context("module path error")?;
        fs::copy(
            module_path,
            format!("{}/Modules/{}", output_path, module_name),
        )?;
    }

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
        fs::remove_dir_all(&xcframework_path)?;
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
