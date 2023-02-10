pub mod cargo;
pub mod lipo;
pub mod targets;
pub mod xcodebuild;

use anyhow::{anyhow, Result};
use std::process::Command;
use yansi::Paint;

fn run_cargo(args: &[String]) -> Result<()> {
    run("cargo", args)
}

fn run(program: &str, args: &[String]) -> Result<()> {
    let mut cmd = Command::new(program).args(args).spawn()?;
    let status = cmd.wait()?;
    let cmd = Paint::new(format!("{} {}", program, args.join(" "))).dimmed();
    if status.success() {
        println!("{} done {}", Paint::green(" XCFramework").bold(), cmd);
        Ok(())
    } else {
        println!(
            "{} error when running: {}",
            Paint::red(" XCFramework").bold(),
            cmd
        );
        Err(anyhow!("Command failed with status: {:?}", status.code()))
    }
}
