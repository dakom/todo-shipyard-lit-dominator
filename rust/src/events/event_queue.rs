use shipyard::*;
use wasm_bindgen::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::dom;

pub fn process_event_queue(world:&mut World, event_queue:&mut Vec<Event>, now:f64) -> Result<(), JsValue> {

    let mut entities_to_delete:Vec<Key> = vec![];

    //process all the events that accumulated since last tick
    for event in event_queue.iter() {
        match event {
            Event::AddTodo(label) => {
                let mut key:Option<Key> = None;
                world.run::<(EntitiesMut, &mut Item, &mut Dirty), _>(|(mut entities, mut item, mut dirty)| {
                    key = Some(entities.add_entity((&mut item, &mut dirty), (Item(label.clone()), Dirty{})));
                });

                if let Some(key) = key {
                    world.run::<(EntitiesMut, &mut Order, &mut Dirty), _>(|(ref mut entities, ref mut orders, ref mut dirty)| {
                        if let Some(order) = orders.iter().next() {
                            order.0.push(key);
                        } else {
                            entities.add_entity((orders, dirty), (Order(Vec::new()), Dirty{}));
                        }
                    });
                }
            },
            Event::ChangePage(page_type) => {
                //this event doesn't affect the ecs at all, just the dom, so do immediately (not in a system)
                dom::page::change_page(*page_type)?;
            },
            _ => {
                log::info!("unhandled Event!")
            }
        }
    }

    //clear the queue
    event_queue.clear();

    //delete the entities that were accumulated during event processing
    world.run::<(EntitiesMut, AllStorages), _>(|(mut entities, mut all_storages)| {
        for id in entities_to_delete {
            entities.delete(&mut all_storages, id);
        }
    });

    Ok(())
}