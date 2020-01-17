use shipyard::prelude::*;
use shipyard::internal::{EntitiesViewMut, ViewMut};
use crate::components::*;
use futures_signals:: {
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::{SignalVec, SignalVecExt, MutableVec, MutableSignalVec},
    map_ref
};
use dominator::clone;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use crate::components::*;
use crate::actions;
use crate::signals;
use crate::storage;
use crate::events::{DropSide};
use shipyard::prelude::*;
use web_sys::{window, Storage};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

pub fn add_todo(world:&World, label:&str, complete:bool) {
    world.run::<((EntitiesMut, &mut Label, &mut Complete), Unique<&List>), _, _>(|(mut views, list)| {
        let (mut entities, labels, completes) = views;
        let entity = entities.add_entity((labels, completes), 
            (
                Label(Mutable::new(label.to_string())), 
                Complete(Mutable::new(complete))
            ));

        list.0.lock_mut().push(entity);
    });
}

pub fn remove_todo(world:&World, id:EntityId) {
    world.run::<(Unique<&List>), _, _>(|list| {
        let mut list = list.0.lock_mut();
        list.retain(|list_id| *list_id != id);
    });
    world.run::<AllStorages, _, _>(|mut all_storages| {
        all_storages.delete(id);
    });
}

pub fn change_filter(world:&World, filter_type:FilterType) {
    world.run::<(Unique<&Filter>), _, _>(|filter| {
        *filter.0.lock_mut() = filter_type;
    });
}

pub fn toggle_todo(world:&World, id:EntityId, complete:bool) {
    world.run::<&Complete, _, _>(|completes| {
        *(&completes).get(id).unwrap().0.lock_mut() = complete;
    });
}

pub fn change_todo(world:&World, id:EntityId, label:&str) {
    world.run::<&mut Label, _, _>(|labels| {
        *(&labels).get(id).unwrap().0.lock_mut() = label.to_string();
    });
}

pub fn reposition(world:&World, src:EntityId, dest:EntityId, side:DropSide) {
    world.run::<Unique<&List>, _, _>(|list| {
        let indices = {    
            let list = list.0.lock_ref();
            let list = list.as_slice();

            (list.iter().position(|id| *id == src), list.iter().position(|id| *id == dest))
        };

        if let (Some(src_index), Some(dest_index)) = indices {
            let dest_index = match side {
                DropSide::Before =>  {
                    if src_index > dest_index {
                        dest_index
                    } else {
                        dest_index-1
                    }
                },
                DropSide::After => {
                    if src_index < dest_index {
                        dest_index
                    } else {
                        dest_index+1
                    }
                },
                _ => src_index 
            };

            if src_index != dest_index {
                list.0.lock_mut().move_from_to(src_index, dest_index);
            }
        }
    });
}

pub fn toggle_all_todos(world:&World, complete: bool) {
    world.run::<&Complete, _, _>(|completes| {
        completes.iter().for_each(|item_complete| {
            *item_complete.0.lock_mut() = complete;
        });
    });
}

pub fn clear_completed(world:&World) {
    let completed_ids:Vec<EntityId> = world.run::<&Complete, _, _>(|completes| {
        (&completes)
            .iter()
            .filter(|complete| complete.0.get())
            .with_id()
            .map(|(id, _)| id)
            .collect()
    });

    world.run::<(Unique<&List>), _, _>(|list| {
        let mut list = list.0.lock_mut();
        list.retain(|list_id| !completed_ids.contains(list_id));
    });

    world.run::<AllStorages, _, _>(|mut all_storages| {
        for id in completed_ids.iter() {
            all_storages.delete(*id);
        }
    });
}

pub fn load(world:Rc<World>) {

    world.run::<Unique<&Phase>, _, _>(|mut phase| {
        *phase.0.lock_mut() = PhaseType::Loading;
    });

    let cloned_world = world.clone();

    let load_future = async move {
        if let Some((data, data_str)) = storage::load_stored_data() {
            data.items.iter().for_each(|(label, completed)| {
                actions::add_todo(&cloned_world, label, *completed);
            });
        } else {
            log::info!("no saved data - new session!");
        }
    };

    spawn_local(async move {
        load_future.await;

        //Need to delay this so that the save listener doesn't see Ready until after the data settles
        //TODO - actually delay the call rather than re-spawning, e.g. await on a Future created via setTimeout
        spawn_local(async move {
            world.run::<Unique<&Phase>, _, _>(|phase| {
                *phase.0.lock_mut() = PhaseType::Ready;
            });
        });
    });
}

pub fn spawn_save_listener(world:Rc<World>) {

    let future = 
        signals::all_items(world.clone())
            .for_each(clone!(world => move |_diff| {
                //We could actually do really efficient saving by using the diff
                //Instead, just let the system check for a wholesale save every second or so
                world.run::<(EntitiesMut, Unique<&Phase>, Unique<&mut SaveTag>), _, _>(|(entities, phase, save_tag)| {
                    match phase.0.get() {
                        PhaseType::Ready => {
                            save_tag.0 = true;
                        },
                        _ => {}
                    }
                });
                async {()}
            }));
    spawn_local(future);
}