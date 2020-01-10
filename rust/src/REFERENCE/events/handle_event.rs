use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::dom::storage;
use std::collections::VecDeque;

/*
pub fn process_event(world:&mut World, event_queue:&mut VecDeque<Event>, event:Event) -> Result<(), JsValue> {
    match event {
        Event::FilterChange(_filter) => {
            world.run::<Unique<&mut Filter>, _, _>(|mut filter| {
                *filter = _filter;
            });
        },
        Event::ClearCompleted => {
            let entity_ids:Vec<EntityId> = world.run::<(&ItemLabel, &ItemComplete), _, _>(|(labels, completes)| {
                (&labels, completes)
                    .iter()
                    .with_id()
                    .filter(|(_, (_, complete))| complete.0)
                    .map(|(id, _)| id)
                    .fold(Vec::new(), |mut vec, x| {vec.push(x); vec})
            });
            world.run::<AllStorages, _, _>(|mut all_storages| {
                entity_ids.iter().for_each(|entity| {
                    all_storages.delete(*entity);
                });
            });
        },
        _ => unimplemented!()
    }
    Ok(())
}
*/