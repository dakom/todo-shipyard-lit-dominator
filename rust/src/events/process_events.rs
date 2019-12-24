use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::context::AppContext;
use crate::dom::storage;

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
            Event::RemoveTodo(entity_id) => {
                world.run::<AllStorages, _, _>(|mut all_storages| {
                    all_storages.delete(*entity_id);
                    mark_item_list_dirty = true;
                });
            },

            Event::SetCompleted(entity_id, completed) => {
                //toggle completed
                world.run::<&mut ItemComplete, _, _>(|ref mut item_completes| {
                    if let Some(item_complete) = (item_completes).get(*entity_id).iter_mut().next() {
                        item_complete.0 = *completed;
                    }
                });
                
                //add dirty tagged component
                world.run::<(EntitiesMut, &mut DirtyTag), _, _>(|(ref mut entities, ref mut dirty_tags)| {
                    entities.add_component(dirty_tags, DirtyTag{}, *entity_id); 
                });
            },
            Event::SetCompletedAll(completed) => {
                world.run::<(Entities, &mut ItemComplete, &mut DirtyTag), _, _>(|(entities, item_completes, mut dirty_tags)| {
                    item_completes.iter().with_id().for_each(|(id, item_complete)| {
                        item_complete.0 = *completed;
                        entities.add_component(&mut dirty_tags, DirtyTag{}, id); 
                    });
                });
            },
            Event::FilterChange(_filter) => {
                world.run::<(Unique<&mut Filter>, Unique<&mut DirtyFilter>), _, _>(|(mut filter, mut dirty_filter)| {
                    *filter = *_filter;
                    dirty_filter.0 = true;
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
                mark_item_list_dirty = true;
            },
            Event::InitialLoad => {
                if let Some((data, data_str)) = storage::load_stored_data()? {
                    world.run::<(EntitiesMut, &mut ItemLabel, &mut ItemComplete, &mut DirtyTag), _, _>(|(mut entities, mut item_labels, mut item_completes, mut dirty_tags)| {
                        data.items.iter().for_each(|(label, completed)| {
                            entities.add_entity((&mut item_labels, &mut item_completes), (ItemLabel(label.to_string()), ItemComplete(*completed)));
                        });
                    });

                    mark_item_list_dirty = true;

                    log::info!("got data! {}", &data_str);
                } else {
                    log::info!("no saved data!");
                }
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