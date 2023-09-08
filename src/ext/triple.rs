use target_lexicon::{triple, Triple};

lazy_static::lazy_static! {
  static ref IMPLICIT_SIM: Vec<Triple> = vec![triple!("x86_64-apple-tvos"), triple!("x86_64-apple-ios")];

}

pub trait TripleExt {
    fn is_apple_simulator(&self) -> bool;
}

impl TripleExt for Triple {
    fn is_apple_simulator(&self) -> bool {
        IMPLICIT_SIM.contains(self) || self.environment == target_lexicon::Environment::Sim
    }
}
