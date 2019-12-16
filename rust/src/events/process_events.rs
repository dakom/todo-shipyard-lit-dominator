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
                    entities.add_entity((&mut item_labels, &mut item_completes, &mut dirty_tags), (ItemLabel(label.to_string()), ItemComplete(false), DirtyTag{}));
                });

                mark_item_list_dirty = true;
            },
            //TODO - use proper key id. See https://github.com/leudz/shipyard/issues/23
            /*
            Event::RemoveTodo(key) => {
                world.run::<AllStorages, _, _>(|mut all_storages| {
                    all_storages.delete(*key);
                });
                mark_item_list_dirty = true;
            },
            */
            Event::RemoveTodo(index) => {
                let key = get_item_key_from_index(world, *index);
                if let Some(key) = key {
                    world.run::<AllStorages, _, _>(|mut all_storages| {
                        all_storages.delete(key);
                        mark_item_list_dirty = true;
                    });
                }
            },

            //TODO - use proper key id. See https://github.com/leudz/shipyard/issues/23
            Event::SetTodoCompleted(index, completed) => {
                let key = get_item_key_from_index(world, *index);

                if let Some(key) = key {
                    world.run::<(EntitiesMut, &mut ItemList, &mut ItemComplete, &mut DirtyTag), _, _>(|(mut entities, mut item_lists, mut item_completes, mut dirty_tags)| {
                        for item_complete in item_completes.iter() {
                            item_complete.0 = *completed;
                        }
                        /*TODO - make it work!
                        for item_complete in item_completes.get(key).iter() {
                            let mut foo = *item_complete;
                            (*foo).0 = false;
                        }
                        */
                    });

                    world.run::<(EntitiesMut, &mut DirtyTag), _, _>(|(ref mut entities, ref mut dirty_tags)| {
                        entities.add_component(dirty_tags, DirtyTag{}, key); 
                    });
                }
            },
            Event::FilterChange(_filter) => {
                world.run::<(Unique<&mut Filter>, Unique<&mut DirtyFilter>), _, _>(|(mut filter, mut dirty_filter)| {
                    *filter = *_filter;
                    dirty_filter.0 = true;
                });
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

//TODO - use proper key id. See https://github.com/leudz/shipyard/issues/23
//When that's implemented, delete this function completely
fn get_item_key_from_index(world:&World, index:usize) -> Option<Key> {
    world.run::<&ItemLabel, _, _>(|item_labels| {
        let result = item_labels.iter().with_id().enumerate().find(|(item_index, _)| {
            *item_index == index
        });

        if let Some((_, (key, _))) = result {
            Some(key)
        } else {
            None
        }
    })
}