use std::process::Command;

use camino::Utf8PathBuf;
use cargo_xcframework::XcCli;
use clap::Parser;
use tempfile::tempdir;

#[test]
fn test_hello() {
    let cli = XcCli::parse_from(&[
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=tests/project/Cargo.toml",
    ]);

    let res = cargo_xcframework::run(cli);
    res.unwrap()
}

#[test]
fn end_to_end_static() {
    let tmp = tempdir().unwrap();
    let tmp_dir = tmp.path().to_str().unwrap();

    let cli = XcCli::parse_from(&[
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=examples/end-to-end/mymath-lib/Cargo.toml",
        "--lib-type=staticlib",
        "--target-dir",
        tmp_dir,
    ]);

    cargo_xcframework::run(cli).unwrap();

    let build_dir = Utf8PathBuf::from("examples/end-to-end/swift-exe/.build");
    if build_dir.exists() {
        fs_err::remove_dir_all(build_dir).unwrap();
    }

    let cmd = Command::new("swift")
        .current_dir("examples/end-to-end/swift-exe")
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
    let tmp_dir = tmp.path().to_str().unwrap();

    let cli = XcCli::parse_from(&[
        "cargo-xcframework",
        "--quiet",
        "--manifest-path=examples/end-to-end/mymath-lib/Cargo.toml",
        "--lib-type=cdylib",
        "--target-dir",
        tmp_dir,
    ]);

    cargo_xcframework::run(cli).unwrap();

    let build_dir = Utf8PathBuf::from("examples/end-to-end/swift-exe/.build");
    if build_dir.exists() {
        fs_err::remove_dir_all(build_dir).unwrap();
    }

    let cmd = Command::new("swift")
        .current_dir("examples/end-to-end/swift-exe")
        .arg("run")
        .output()
        .unwrap();

    let output = String::from_utf8_lossy(&cmd.stdout);
    eprintln!("{}", String::from_utf8_lossy(&cmd.stderr));

    assert_eq!("MyMath.rust_add(4 + 2) = 6\n", output);
}
