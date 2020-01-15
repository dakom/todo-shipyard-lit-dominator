use shipyard::prelude::*;
use crate::components::*;
use crate::storage;

pub const SAVE:&'static str = "SAVE";

pub fn register_workloads(world:&World) {
    world.add_workload(SAVE, Save); 
}

#[system(Save)]
pub fn run (save_tag:Unique<&mut SaveTag>, list:Unique<&List>, labels:&Label, completes:&Complete) {
    if save_tag.0 {
        log::info!("SAVING!....");
        let items:Vec<(String, bool)> = 
            list.0.lock_ref().iter().map(|id| {
                let (label, complete) = (&labels, &completes).get(*id).unwrap();
                (label.0.get_cloned(), complete.0.get())
            })
            .collect();

        storage::save_stored_data(&storage::StoredData::new(items)).unwrap();
        save_tag.0 = false;
    } 
}