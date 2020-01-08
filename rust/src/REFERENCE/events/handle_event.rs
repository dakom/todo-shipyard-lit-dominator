use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::*;
use crate::events::*;
use crate::dom::storage;
use std::collections::VecDeque;

/*
pub fn process_event(world:&mut World, event_queue:&mut VecDeque<Event>, event:Event) -> Result<(), JsValue> {
    match event {
        Event::AddTodo(label) => {
            world.run::<(EntitiesMut, &mut ItemLabel, &mut ItemComplete), _, _>(|(mut entities, mut item_labels, mut item_completes)| {
                let entity = entities.add_entity((&mut item_labels, &mut item_completes), (ItemLabel(label.to_string()), ItemComplete(false)));
            });
        },

        Event::RemoveTodo(entity_id) => {
            world.run::<AllStorages, _, _>(|mut all_storages| {
                all_storages.delete(entity_id);
            });
        },
        Event::SetCompleted(entity_id, completed) => {
            world.run::<&mut ItemComplete, _, _>(|ref mut item_completes| {
                if let Some(item_complete) = (item_completes).get(entity_id).iter_mut().next() {
                    item_complete.0 = completed;
                }
            });
        },
        Event::ChangeTodo(entity_id, label) => {
            world.run::<&mut ItemLabel, _, _>(|ref mut item_labels| {
                if let Some(item_label) = (item_labels).get(entity_id).iter_mut().next() {
                    item_label.0 = label;
                }
            });
        },
        Event::SetCompletedAll(completed) => {
            world.run::<&mut ItemComplete, _, _>(|item_completes| {
                item_completes.iter().with_id().for_each(|(id, item_complete)| {
                    item_complete.0 = completed;
                });
            });
        },
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
        Event::InitialLoad => {
            if let Some((data, data_str)) = storage::load_stored_data()? {
                world.run::<(EntitiesMut, &mut ItemLabel, &mut ItemComplete), _, _>(|(mut entities, mut item_labels, mut item_completes)| {
                    data.items.iter().for_each(|(label, completed)| {
                        entities.add_entity((&mut item_labels, &mut item_completes), (ItemLabel(label.to_string()), ItemComplete(*completed)));
                    });
                });

                log::info!("got data! {}", &data_str);
            } else {
                log::info!("no saved data!");
            }
        },
        _ => unimplemented!()
    }
    Ok(())
}
*/