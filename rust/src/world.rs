use shipyard::prelude::*;
use crate::components::*;
use crate::context::*;
use crate::systems;

pub fn init_world() -> World {
    let mut world = World::new::<(
        ItemLabel,
        ItemComplete,
    )>();

    world.register_unique(Filter::default());
    world.register_unique(Phase::default());

    world.update_pack::<ItemLabel>();
    world.update_pack::<ItemComplete>();

    systems::workloads::register_workloads(&mut world);

    world
}