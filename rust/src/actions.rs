use shipyard::prelude::*;
//use shipyard::internal::{EntitiesViewMut, ViewMut};
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
    let (mut entities, views, list) = world.borrow::<(EntitiesMut, (&mut Label, &mut Complete), Unique<&List>)>();
    let (mut labels, mut completes) = views;

    let entity = entities.add_entity((&mut labels, &mut completes), 
        (
            Label(Mutable::new(label.to_string())), 
            Complete(Mutable::new(complete))
        ));

    list.0.lock_mut().push(entity);
}

pub fn remove_todo(world:&World, id:EntityId) {
    {
        let list = world.borrow::<Unique<&List>>();
        let mut list = list.0.lock_mut();
        list.retain(|list_id| *list_id != id);
       //do stuff
    }

    world.borrow::<AllStorages>().delete(id);
}

pub fn change_filter(world:&World, filter_type:FilterType) {
    let filter = world.borrow::<Unique<&Filter>>();
    *filter.0.lock_mut() = filter_type;
}

pub fn toggle_todo(world:&World, id:EntityId, complete:bool) {
    let completes = world.borrow::<&Complete>();
    *(&completes).get(id).unwrap().0.lock_mut() = complete;
}

pub fn change_todo(world:&World, id:EntityId, label:&str) {
    let labels = world.borrow::<&mut Label>();
    *(&labels).get(id).unwrap().0.lock_mut() = label.to_string();
}

pub fn reposition(world:&World, src:EntityId, dest:EntityId, side:DropSide) {
    let list = world.borrow::<Unique<&List>>();
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
}

pub fn toggle_all_todos(world:&World, complete: bool) {
    let completes = world.borrow::<&Complete>();
    completes.iter().for_each(|item_complete| {
        *item_complete.0.lock_mut() = complete;
    });
}

pub fn clear_completed(world:&World) {

    let completed_ids = {
        let (completes, list) = world.borrow::<(&Complete, Unique<&List>)>();
        let completed_ids:Vec<EntityId> = 
            (&completes)
                .iter()
                .filter(|complete| complete.0.get())
                .with_id()
                .map(|(id, _)| id)
                .collect();

        let mut list = list.0.lock_mut();
        list.retain(|list_id| !completed_ids.contains(list_id));

        completed_ids
    };

    let mut all_storages = world.borrow::<AllStorages>();
    for id in completed_ids.iter() {
        all_storages.delete(*id);
    }
}

pub fn load(world:Rc<World>) {

    let load_future = {
        let cloned_world = world.clone();
        let phase = world.borrow::<Unique<&Phase>>();
        *phase.0.lock_mut() = PhaseType::Loading;


        async move {
            if let Some((data, data_str)) = storage::load_stored_data() {
                data.items.iter().for_each(|(label, completed)| {
                    actions::add_todo(&cloned_world, label, *completed);
                });
            } else {
                log::info!("no saved data - new session!");
            }
        }
    };

    spawn_local(async move {
        load_future.await;

        //Need to delay this so that the save listener doesn't see Ready until after the data settles
        //TODO - actually delay the call rather than re-spawning, e.g. await on a Future created via setTimeout
        spawn_local(async move {
            let phase = world.borrow::<Unique<&Phase>>();
            *phase.0.lock_mut() = PhaseType::Ready;
        });
    });
}

pub fn spawn_save_listener(world:Rc<World>) {

    let future = 
        signals::all_items(world.clone())
            .for_each(clone!(world => move |_diff| {
                //We could actually do really efficient saving by using the diff
                //Instead, just let the system check for a wholesale save every second or so

                let (phase, mut save_tag) = world.borrow::<(Unique<&Phase>, Unique<&mut SaveTag>)>();

                match phase.0.get() {
                    PhaseType::Ready => {
                        save_tag.0 = true;
                    },
                    _ => {}
                }
                async {()}
            }));
    spawn_local(future);
}