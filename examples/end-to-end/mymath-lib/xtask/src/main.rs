use anyhow::{Context, Result};

fn main() -> Result<()> {
    // TODO: complete cargo-xtask usage example

    let cli = xcframework::XcCli {
        lib_type: None,
        quiet: false,
        package: None,
        verbose: 0,
        unstable_flags: None,
        release: false,
        profile: None,
        features: vec![],
        all_features: false,
        no_default_features: false,
        target_dir: None,
        manifest_path: None,
    };
    let produced = xcframework::build(cli).context("building with xcframework")?;

    println!("produced: {:?}", produced);

    Ok(())
}
