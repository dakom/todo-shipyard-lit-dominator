use shipyard::prelude::*;
use crate::components::*;
use futures_signals::signal_vec::{MutableVec};
use futures_signals::signal::{Mutable};
use lazy_static::lazy_static;

pub fn init_world() -> World {
    let mut world = World::new::<(
        Label,
        Complete,
    )>();

    world.add_unique(List (MutableVec::new()));
    world.add_unique(Filter(Mutable::new(FilterType::default())));
    world.add_unique(Phase(Mutable::new(PhaseType::default())));
    world.add_unique(SaveTag(false));
    world.tight_pack::<(Label, Complete)>();

    world
}