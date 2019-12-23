use shipyard::prelude::*;
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
                world.run::<(EntitiesMut, &mut ItemLabel, &mut ItemComplete, &mut DirtyTag), _, _>(|(mut entities, mut item_labels, mut item_completes, mut dirty_tags)| {
                    let entity = entities.add_entity((&mut item_labels, &mut item_completes), (ItemLabel(label.to_string()), ItemComplete(false)));
                });

                mark_item_list_dirty = true;
            },
            Event::RemoveTodo(key) => {
                world.run::<AllStorages, _, _>(|mut all_storages| {
                    all_storages.delete(*key);
                    mark_item_list_dirty = true;
                });
            },

            Event::SetTodoCompleted(key, completed) => {
                //toggle completed
                world.run::<&mut ItemComplete, _, _>(|ref mut item_completes| {
                    if let Some(item_complete) = (item_completes).get(*key).iter_mut().next() {
                        item_complete.0 = *completed;
                    }
                });
                
                //add dirty tagged component
                world.run::<(EntitiesMut, &mut DirtyTag), _, _>(|(ref mut entities, ref mut dirty_tags)| {
                    entities.add_component(dirty_tags, DirtyTag{}, *key); 
                });
            },
            Event::FilterChange(_filter) => {
                world.run::<(Unique<&mut Filter>, Unique<&mut DirtyFilter>), _, _>(|(mut filter, mut dirty_filter)| {
                    *filter = *_filter;
                    dirty_filter.0 = true;
                });
            },
            Event::ClearCompleted => {
                let entity_ids:Vec<Key> = world.run::<(&ItemLabel, &ItemComplete), _, _>(|(labels, completes)| {
                    (&labels, completes)
                        .iter()
                        .with_id()
                        .filter(|(_, _, complete)| complete.0)
                        .map(|(id, _, _)| id)
                        .collect()
                });
                world.run::<AllStorages, _, _>(|mut all_storages| {
                    for entity in entity_ids.iter() {
                        all_storages.delete(*entity);
                    }
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
        world.run::<(EntitiesMut, &mut DirtyTag), _, _>(|(ref mut entities, ref mut dirty_tags)| {
            entities.add_component(dirty_tags, DirtyTag{}, key_cache.item_list);
        });
    }
    Ok(())
}