use shipyard::prelude::*;
use crate::components::*;
use crate::storage;

pub const SAVE:&'static str = "SAVE";

pub fn register_workloads(world:&World) {
    world.add_workload(SAVE, storage::Save); 
}
