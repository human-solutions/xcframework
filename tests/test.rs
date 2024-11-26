use anyhow::Result;
use camino_fs::*;
use std::process::Command;
use xcframework::CliArgs;

fn args(vec: &[&str]) -> CliArgs {
    CliArgs::from_vec(vec.iter().map(|s| s.into()).collect()).unwrap()
}
#[test]
fn test_hello() {
    // FIXME: if needed targets missing, it will block running the test by interactive prompt
    // WORKAROUND: config `rust-toolchain.toml`, prepare the targets fisrt

    let cli = args(&["--quiet", "--manifest-path", "tests/project/Cargo.toml"]);

    let produced = xcframework::build_from_cli(cli).unwrap();
    assert!(produced.is_zipped);
    assert_eq!(produced.module_name, "HelloTest");
}

fn create_output_dir(subfolder: &str) -> Utf8PathBuf {
    let tmp_dir = Utf8PathBuf::from("tests").join("temp").join(subfolder);
    tmp_dir.rm().unwrap();
    tmp_dir.mkdirs().unwrap();
    tmp_dir
}

#[test]
fn end_to_end_static() {
    let out_dir = create_output_dir("static");

    let target_dir = out_dir.join("mymath-lib/target");
    target_dir.mkdirs().unwrap();

    let cli = args(&[
        "--quiet",
        "--manifest-path",
        "examples/end-to-end/mymath-lib/Cargo.toml",
        "--lib-type",
        "staticlib",
        "--target-dir",
        target_dir.as_str(),
    ]);

    let produced = xcframework::build_from_cli(cli).unwrap();
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
    assert!(cmd.status.success());
    assert_eq!("MyMath.rust_add(4 + 2) = 6\n", stdout);
}

#[test]
fn end_to_end_dynamic() {
    let out_dir = create_output_dir("dynamic");

    let target_dir = out_dir.join("mymath-lib/target");
    target_dir.mkdirs().unwrap();

    let cli = args(&[
        "--quiet",
        "--manifest-path",
        "examples/end-to-end/mymath-lib/Cargo.toml",
        "--lib-type",
        "cdylib",
        "--target-dir",
        target_dir.as_str(),
    ]);

    let produced = xcframework::build_from_cli(cli).unwrap();
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

#[test]
#[ignore = "FIXME: not work on CI"]
fn multi_platform_static() {
    let out_dir = create_output_dir("multi-platform-static");
    let target_dir = out_dir.join("mymath-lib/target");
    target_dir.mkdirs().unwrap();
    let cli = args(&[
        "--manifest-path",
        "examples/multi-platform/mymath-lib/Cargo.toml",
        "--lib-type",
        "staticlib",
        "--target-dir",
        target_dir.as_str(),
    ]);
    let produced = xcframework::build_from_cli(cli).unwrap();
    assert_eq!(produced.module_name, "MyMath");
    let tuist_workspace_dir = cp_tuist_workspace(out_dir.as_path()).unwrap();
    let cmd = Command::new("tuist")
        .current_dir(&tuist_workspace_dir)
        .arg("test")
        .output()
        .unwrap();
    assert!(cmd.status.success());
    if !cmd.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&cmd.stdout));
        eprintln!("{}", String::from_utf8_lossy(&cmd.stderr));
    }
}

#[test]
#[ignore = "FIXME: not work on CI"]
fn multi_platform_dynamic() {
    let out_dir = create_output_dir("multi-platform-dynamic");
    let target_dir = out_dir.join("mymath-lib/target");
    target_dir.mkdirs().unwrap();

    let cli = args(&[
        "--manifest-path",
        "examples/multi-platform/mymath-lib/Cargo.toml",
        "--lib-type",
        "cdylib",
        "--target-dir",
        target_dir.as_str(),
    ]);
    let produced = xcframework::build_from_cli(cli).unwrap();
    assert_eq!(produced.module_name, "MyMath");
    let tuist_workspace_dir = cp_tuist_workspace(out_dir.as_path()).unwrap();
    let cmd = Command::new("tuist")
        .current_dir(&tuist_workspace_dir)
        .arg("test")
        .output()
        .unwrap();
    assert!(cmd.status.success());
    if !cmd.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&cmd.stdout));
        eprintln!("{}", String::from_utf8_lossy(&cmd.stderr));
    }
}

fn cp_swift_exe(dest: &Utf8Path) -> Result<Utf8PathBuf> {
    println!("dest: {:?}", dest);
    let from = Utf8PathBuf::from("examples/end-to-end/swift-exe");

    let dest = dest.join("swift-exe");
    dest.mkdirs()?;

    from.cp(&dest)?;
    let build_tmp = dest.join(".build");
    build_tmp.rm()?;
    Ok(dest)
}

fn cp_tuist_workspace(dest: &Utf8Path) -> Result<Utf8PathBuf> {
    let from = Utf8PathBuf::from("examples/multi-platform/tuist-workspace");
    dest.mkdirs()?;
    from.cp(dest)?;
    Ok(dest.join("tuist-workspace"))
}
