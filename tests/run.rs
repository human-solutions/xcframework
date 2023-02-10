use cargo_xcframework::Cli;
use clap::Parser;

#[test]
fn test_hello() {
    let cli = Cli::parse_from(&[
        "cargo-xcframework",
        "--manifest-path=tests/project/Cargo.toml",
    ]);

    let res = cargo_xcframework::run(cli);
    assert!(res.is_ok())
}
