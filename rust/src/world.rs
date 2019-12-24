use shipyard::prelude::*;
use crate::components;
use crate::context::*;
use crate::systems;
use crate::components::{ItemList, DirtyTag};

pub fn init_world() -> (World, KeyCache) {
    let mut world = World::new::<(
        components::ItemLabel,
        components::ItemComplete,
        components::ItemList,
        components::DirtyTag,
    )>();

    world.register_unique(components::Filter::default());
    world.register_unique(components::DirtyFilter::default());
    world.register_unique(components::Phase::default());

    systems::register_workloads(&mut world);

    let mut item_list_id:Option<EntityId> = None;
    world.run::<(EntitiesMut, &mut ItemList, &mut DirtyTag), _, _>(|(mut entities, mut item_lists, mut dirties)| {
        item_list_id = Some(entities.add_entity((&mut item_lists, &mut dirties), (ItemList {}, DirtyTag {}) ));
    });

    (world, KeyCache { 
        item_list: item_list_id.unwrap(),
    })
}