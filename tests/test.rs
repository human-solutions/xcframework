use anyhow::Result;
use camino::Utf8PathBuf;
use clap::Parser;
use fs_err as fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use xcframework::ext::PathBufExt;
use xcframework::XcCli;

#[test]
fn test_hello() {
    // FIXME: if needed targets missing, it will block running the test by interactive prompt
    // WORKAROUND: config `rust-toolchain.toml`, prepare the targets fisrt

    let cli = XcCli::parse_from([
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=tests/project/Cargo.toml",
    ]);

    // FIXME: if needed targets missing, it will block running the test by interactive prompt
    // WORKAROUND: add the targets fisrt
    rustup_configurator::target::install(&vec![
        "aarch64-apple-darwin".to_owned(),
        "aarch64-apple-ios".to_owned(),
        "aarch64-apple-ios-sim".to_owned(),
        "x86_64-apple-darwin".to_owned(),
        "x86_64-apple-ios".to_owned(),
    ]).unwrap();

    let produced = xcframework::build(cli).unwrap();
    assert!(produced.is_zipped);
    assert_eq!(produced.module_name, "HelloTest");
}

fn create_output_dir(subfolder: &str) -> PathBuf {
    let tmp_dir = PathBuf::from("tests").join("temp").join(subfolder);
    if tmp_dir.exists() {
        fs::remove_dir_all(&tmp_dir).unwrap();
    }
    fs::create_dir_all(&tmp_dir).unwrap();
    tmp_dir
}

#[test]
fn end_to_end_static() {
    let out_dir = create_output_dir("static");

    let target_dir = out_dir.join("mymath-lib/target");
    fs::create_dir_all(&target_dir).unwrap();

    let cli = XcCli::parse_from([
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=examples/end-to-end/mymath-lib/Cargo.toml",
        "--lib-type=staticlib",
        "--target-dir",
        &target_dir.to_str().unwrap(),
    ]);

    let produced = xcframework::build(cli).unwrap();
    assert!(produced.is_zipped);
    assert_eq!(produced.module_name, "MyMath");

    let swift_dir = cp_swift_exe(&out_dir).unwrap();

    let cmd = Command::new("swift")
        .current_dir(swift_dir)
        .arg("run")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&cmd.stdout);
    let stderr = String::from_utf8_lossy(&cmd.stderr);
    eprintln!("{stderr}");
    assert!(stderr.contains("Build complete!"));
    assert_eq!("MyMath.rust_add(4 + 2) = 6\n", stdout);
}

#[test]
fn end_to_end_dynamic() {
    let out_dir = create_output_dir("dynamic");

    let target_dir = out_dir.join("mymath-lib/target");
    fs::create_dir_all(&target_dir).unwrap();

    let cli = XcCli::parse_from([
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=examples/end-to-end/mymath-lib/Cargo.toml",
        "--lib-type=cdylib",
        "--target-dir",
        &target_dir.to_str().unwrap(),
    ]);

    let produced = xcframework::build(cli).unwrap();
    assert!(produced.is_zipped);
    assert_eq!(produced.module_name, "MyMath");

    let swift_dir = cp_swift_exe(out_dir.as_path()).unwrap();
    let cmd = Command::new("swift")
        .current_dir(&swift_dir)
        .arg("run")
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&cmd.stdout);
    eprintln!("{}", String::from_utf8_lossy(&cmd.stderr));

    assert_eq!("MyMath.rust_add(4 + 2) = 6\n", output);
}

fn cp_swift_exe(dest: &Path) -> Result<Utf8PathBuf> {
    let from = Utf8PathBuf::from("examples/end-to-end/swift-exe");

    let dest = Utf8PathBuf::from_path_buf(dest.to_path_buf()).unwrap();

    dest.create_dir_all_if_needed()?;

    fs_extra::dir::copy(from, &dest, &fs_extra::dir::CopyOptions::new())?;
    let build_tmp = dest.join("swift-exe/.build");
    if build_tmp.exists() {
        fs::remove_dir_all(build_tmp)?;
    }
    Ok(dest.join("swift-exe"))
}
