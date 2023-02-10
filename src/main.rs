use cargo_xcframework::{run, Cli};
use clap::Parser;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    // when running as cargo leptos, the second argument is "leptos" which
    // clap doesn't expect
    if args.get(1).map(|a| a == "uniffi-package").unwrap_or(false) {
        args.remove(1);
    }

    let args = Cli::parse_from(&args);
    if let Err(e) = crate::run(args) {
        eprintln!("{:?}", e);
        std::process::exit(1);
    }
}
