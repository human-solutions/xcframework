use anyhow::Result;

use crate::conf::Configuration;

pub fn build(conf: &Configuration) -> Result<()> {
    let mut args: Vec<String> = vec![];

    args.push("build".into());
    args.push("--color=always".into());

    if conf.target_dir != "target" {
        args.push(format!("--target-dir={}", conf.target_dir));
    }

    if let Some(manifest_path) = &conf.cli.manifest_path {
        args.push(format!("--manifest-path={manifest_path}"));
    }
    if conf.cli.quiet {
        args.push("--quiet".into());
    }

    if let Some(package) = &conf.cli.package {
        args.push(format!("--package={package}"));
    }

    for _ in 0..conf.cli.verbose {
        args.push("-v".into());
    }

    if let Some(flags) = &conf.cli.unstable_flags {
        args.push(format!("-Z={flags}"));
    }

    if conf.cli.release {
        args.push("--release".into());
    }

    if let Some(profile) = &conf.cli.profile {
        args.push(format!("--profile={profile}"));
    }

    if !conf.cli.features.is_empty() {
        args.push(format!("--features={}", conf.cli.features.join(",")));
    }

    if conf.cli.all_features {
        args.push("--all-features".into());
    }

    if conf.cli.no_default_features {
        args.push("--no-default-features".into());
    }

    for target in conf.cargo_section.chosen_targets() {
        args.push(format!("--target={}", target));
    }
    super::run_cargo(&args, conf.cli.quiet)
}
