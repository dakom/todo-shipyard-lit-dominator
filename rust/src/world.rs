use shipyard::prelude::*;
use crate::components::*;
use crate::systems;
use futures_signals::signal_vec::{MutableVec};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref WORLD:World = init_world(); 
}

pub fn init_world() -> World {
    let mut world = World::new::<(
        ItemLabel,
        ItemComplete,
    )>();

    world.add_unique(Filter::default());
    world.add_unique(Phase::default());
    world.add_unique(TodoList(MutableVec::new()));

    world.update_pack::<ItemLabel>();
    world.update_pack::<ItemComplete>();

    systems::workloads::register_workloads(&mut world);

    world
}