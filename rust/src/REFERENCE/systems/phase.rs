use shipyard::prelude::*;
use crate::components::{Phase, PhaseType};

#[system(PhaseReady)]
pub fn run (phase:Unique<&'a mut Phase>) {
    *phase.0.lock_mut() = PhaseType::Ready
}