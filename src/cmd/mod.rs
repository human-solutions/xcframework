pub mod cargo;
pub mod modulemap;

use anyhow::{anyhow, Result};
use std::process::Command;
use yansi::Paint;

fn run_cargo(args: &[String], quiet: bool) -> Result<()> {
    run("cargo", args, quiet)
}

fn run(program: &str, args: &[String], quiet: bool) -> Result<()> {
    if quiet {
        return run_quiet(program, args);
    }

    let mut cmd = Command::new(program).args(args).spawn()?;
    let status = cmd.wait()?;
    let cmd = Paint::new(format!("{} {}", program, args.join(" "))).dim();
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

fn run_quiet(program: &str, args: &[String]) -> Result<()> {
    let output = Command::new(program).args(args).output()?;

    let cmd = Paint::new(format!("{} {}", program, args.join(" "))).dim();
    if output.status.success() {
        Ok(())
    } else {
        println!(
            "{} error when running: {}",
            Paint::red(" XCFramework").bold(),
            cmd
        );
        eprintln!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        Err(anyhow!(
            "Command failed with status: {:?}",
            output.status.code()
        ))
    }
}
