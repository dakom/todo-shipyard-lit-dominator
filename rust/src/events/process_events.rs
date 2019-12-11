use shipyard::*;
use wasm_bindgen::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::context::AppContext;

pub fn process_events(app_ctx:&mut AppContext, _now:f64) -> Result<(), JsValue> {
    let AppContext {event_queue, world, key_cache} = app_ctx;

    //if the list as a whole needs to be entirely re-rendered
    let mut mark_item_list_dirty = false;

    //process all the events that accumulated since last tick
    for event in event_queue.iter() {
        match event {
            Event::AddTodo(label) => {
                world.run::<(EntitiesMut, &mut Item), _>(|(mut entities, mut items)| {
                    entities.add_entity(&mut items, Item::new(label));
                });

                mark_item_list_dirty = true;
            },
            Event::RemoveTodo(key) => {
                world.run::<(EntitiesMut, AllStorages), _>(|(mut entities, mut all_storages)| {
                    entities.delete(&mut all_storages, *key);
                });
                mark_item_list_dirty = true;
            },
            _ => {
                log::info!("unhandled Event!")
            }
        }
    }

    //clear the queue
    event_queue.clear();

    //Mark the item list dirty
    if mark_item_list_dirty {
        world.run::<(EntitiesMut, &mut Dirty), _>(|(ref mut entities, ref mut dirties)| {
            entities.add_component(dirties, Dirty{}, key_cache.item_list);
        });
    }
    Ok(())
}