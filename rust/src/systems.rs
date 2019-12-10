use shipyard::*;
use crate::components::*;
use crate::events::*;
use crate::dom;
use wasm_bindgen::prelude::*;

pub fn run_systems_tick(world:&mut World, event_queue:&mut Vec<Event>, now:f64) -> Result<(), JsValue> {
    //run the render system
    world.run_workload("Render");

    Ok(())
}

#[system(RenderDirtyOrder)]
pub fn run (order:&Order, items:&Item, dirty:&Dirty) {
    //TODO - maybe don't even maintain separate order
    //See: https://github.com/leudz/shipyard/blob/85acb2b8b39b6e4c252bba3140a5ff1c54d30a62/src/lib.rs#L1217
    if let Some((order, _dirty)) = (order, dirty).iter().next() {
        //TODO - would be nice to avoid the copy
        let item_and_keys:Vec<(Key, &str)> = 
            items
                .iter()
                .with_id()
                .map(|(id, val)| 
                    (id, val.0.as_ref())
                )
                .collect();

        let data:Vec<&str> = 
            order.0.iter()
                .map(|key| {
                    match item_and_keys.iter().find(|(id, val)| id == key) {
                        None => "",
                        Some((_, value)) => value
                    }
                })
                .collect();

        dom::items::rewrite_items(&data).unwrap();
    }
    // for item in items.iter() {
    //     log::info!("in system! {}", item.0)
    // }
}
#[system(RenderDirtyItems)]
pub fn run (items:&Item, dirty:&Dirty) {
    //let data:Vec<&str> = items.iter().map(|x| x.0.as_ref()).collect();
    /*
    for item in items.iter() {
        //log::info!("in system! {}", item.0)
    }
    */
}

#[system(RenderClearDirty)]
pub fn run (mut dirties:&mut Dirty) {
    let entity_ids:Vec<Key> = 
        (&dirties)
            .iter()
            .with_id()
            .map(|(id, _)| id)
            .collect();

    for id in entity_ids {
        Remove::<(Dirty,)>::remove((&mut dirties,), id);
    }
}

fn propogate_events(events: impl Iterator<Item=Event>) {
    for event in events.into_iter() {
        log::info!("event..");
    }
    // world.run::<(EntitiesMut, &mut Position, &mut Health), _>(|(mut entities, mut pos, mut health)| {
    //     entities.add_entity((&mut pos, &mut health), (Position { x: 0.0, y: 0.0 }, Health(1000.0)));
    // });
}