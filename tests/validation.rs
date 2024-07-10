use anyhow::bail;
use anyhow::Result;
use rustup_configurator::target::Triple;
use std::str::FromStr;
use xcframework::ext::TripleExt;

fn validate_triples(
    targets: &Vec<Triple>,
    os: &target_lexicon::OperatingSystem,
    simulator: bool,
) -> Result<()> {
    for triple in targets {
        let triple = target_lexicon::Triple::from_str(triple)
            .unwrap_or_else(|_| panic!("Triple is invalid: {triple}"));
        if triple.operating_system != *os {
            bail!("expected {os} not {} in {triple}", triple.architecture);
        }
        use target_lexicon::Vendor::Apple;
        if triple.vendor != Apple {
            bail!("expected {Apple} not {} in {triple}", triple.architecture,);
        }
        if simulator && !triple.is_apple_simulator() {
            bail!("expected a simulator architecture not {triple}");
        }
    }
    Ok(())
}
