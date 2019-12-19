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

    systems::register_workloads(&mut world);

    let mut item_list_key:Option<Key> = None;
    world.run::<(EntitiesMut, &mut ItemList, &mut DirtyTag), _, _>(|(mut entities, mut item_lists, mut dirties)| {
        item_list_key = Some(entities.add_entity((&mut item_lists, &mut dirties), (ItemList {}, DirtyTag {}) ));
    });

    //execute test
    browser_test();

    (world, KeyCache { 
        item_list: item_list_key.unwrap(),
    })
}

//for running in a browser (cargo_test will be stripped)
fn browser_test() {
    run_test();
}

#[test]
fn cargo_test() {
    run_test();
}

fn run_test() {
    let mut world = World::default();
    world.register::<u32>();
    world.register::<usize>();
    //create a few entities
    let (key0, key1, key2) = world.run::<(EntitiesMut, &mut u32, &mut usize), _, _>(|(mut entities, mut u32s, mut usizes)| {
        (
            entities.add_entity((&mut u32s, &mut usizes), (0, 0)),
            entities.add_entity((&mut u32s, &mut usizes), (1, 1)),
            entities.add_entity((&mut u32s, &mut usizes), (2, 2)),
        )
    });

    //delete the first entity
    world.run::<AllStorages, _, _>(|mut all_storages| {
        assert!(all_storages.delete(key0));
    });

    //add a new entity
    let key3 = world.run::<(EntitiesMut, &mut u32, &mut usize), _, _>(|(mut entities, mut u32s, mut usizes)| {
        entities.add_entity((&mut u32s, &mut usizes), (3, 3))
    });

    let entity_str = format!("{:?}", key3);
    let expected_str = "Key { index: 0, version: 1 }";


    assert_eq!(entity_str, expected_str);
}

