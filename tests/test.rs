use anyhow::Result;
use camino::Utf8PathBuf;
use cargo_xcframework::XcCli;
use clap::Parser;
use std::process::Command;
use tempfile::{tempdir, TempDir};

#[test]
fn test_hello() {
    let cli = XcCli::parse_from(&[
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=tests/project/Cargo.toml",
    ]);

    let produced = cargo_xcframework::build(cli).unwrap();
    assert!(produced.is_zipped);
    assert_eq!(produced.module_name, "HelloTest");
}

#[test]
fn end_to_end_static() {
    let tmp = tempdir().unwrap();
    let target_dir = tmp
        .path()
        .join("mymath-lib/target")
        .to_str()
        .unwrap()
        .to_string();

    let cli = XcCli::parse_from(&[
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=examples/end-to-end/mymath-lib/Cargo.toml",
        "--lib-type=staticlib",
        "--target-dir",
        &target_dir,
    ]);

    let swift_dir = cp_swift_exe(&tmp).unwrap();
    let produced = cargo_xcframework::build(cli).unwrap();
    assert!(produced.is_zipped);
    assert_eq!(produced.module_name, "MyMath");

    let cmd = Command::new("swift")
        .current_dir(swift_dir)
        .arg("run")
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&cmd.stdout);
    eprintln!("{}", String::from_utf8_lossy(&cmd.stderr));

    assert_eq!("MyMath.rust_add(4 + 2) = 6\n", output);
}

#[test]
fn end_to_end_dynamic() {
    let tmp = tempdir().unwrap();
    let target_dir = tmp
        .path()
        .join("mymath-lib/target")
        .to_str()
        .unwrap()
        .to_string();

    let cli = XcCli::parse_from(&[
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=examples/end-to-end/mymath-lib/Cargo.toml",
        "--lib-type=cdylib",
        "--target-dir",
        &target_dir,
    ]);

    let swift_dir = cp_swift_exe(&tmp).unwrap();

    let produced = cargo_xcframework::build(cli).unwrap();
    assert!(produced.is_zipped);
    assert_eq!(produced.module_name, "MyMath");

    let cmd = Command::new("swift")
        .current_dir(&swift_dir)
        .arg("run")
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&cmd.stdout);
    eprintln!("{}", String::from_utf8_lossy(&cmd.stderr));

    assert_eq!("MyMath.rust_add(4 + 2) = 6\n", output);
}

fn cp_swift_exe(tmp: &TempDir) -> Result<Utf8PathBuf> {
    let from = Utf8PathBuf::from("examples/end-to-end/swift-exe");

    let to = Utf8PathBuf::from_path_buf(tmp.path().to_path_buf()).unwrap();

    if !to.exists() {
        fs_err::create_dir_all(&to)?;
    }
    fs_extra::dir::copy(&from, &to, &fs_extra::dir::CopyOptions::new())?;
    Ok(to.join("swift-exe"))
}
