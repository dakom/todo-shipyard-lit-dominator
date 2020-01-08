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
use shipyard::prelude::*;
use web_sys::{window, Storage};
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;

pub fn load(world:Rc<World>) {

    world.run::<Unique<&Phase>, _, _>(|mut phase| {
        *phase.0.lock_mut() = PhaseType::Loading;
    });

    let cloned_world = world.clone();

    let load_future = async move {
        if let Some((data, data_str)) = load_stored_data() {
            cloned_world.run::<((EntitiesMut, &mut Label, &mut Complete), Unique<&List>), _, _>(|(mut views, list)| {
                data.items.iter().for_each(|(label, completed)| {
                    actions::add_todo((&mut views, &list.0), label, *completed);
                });
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
#[system(Save)]
pub fn run (save_tag:Unique<&mut SaveTag>, labels:&Label, completes:&Complete) {
    if save_tag.0 {
        log::info!("SAVING!....");
        let items:Vec<(String, bool)> = 
            (&labels, &completes)
                .iter()
                .map(|tuple| ((tuple.0).0.get_cloned(), (tuple.1).0.get()))
                .fold(Vec::new(), |mut v, x| { v.push(x); v });

        save_stored_data(&StoredData::new(items)).unwrap();
    } else {
        //log::info!("NOT SAVING...");
    }

    save_tag.0 = false;
    //TODO
    /*
    // phase is gated so that we don't save the initial empty data 
    if *phase == Phase::Ready {
        if (&item_lists, &dirty_tags).iter().next().is_some() 
            || (&item_completes, &dirty_tags).iter().next().is_some()
            || (&item_labels, &dirty_tags).iter().next().is_some()
            {
                let items:Vec<(&str, bool)> = 
                    (&item_labels, &item_completes)
                        .iter()
                        .map(|tuple| ((tuple.0).0.as_ref(), (tuple.1).0))
                        .fold(Vec::new(), |mut v, x| { v.push(x); v });

                save_stored_data(&StoredData::new(items)).unwrap();
        }
    } else {
        *phase = Phase::Ready;
    }
    */
}

#[derive(Serialize, Deserialize)]
struct StoredData {
    pub items: Vec<(String, bool)>
}

impl StoredData {
    pub fn new<I: IntoIterator<Item=(String, bool)>>(items: I) -> Self {
        Self {
            items: items.into_iter().map(|tuple| (tuple.0, tuple.1)).collect()
        }
    }
}

const STORAGE_NAME:&'static str = "todos-shipyard-lit";

fn load_stored_data() -> Option<(StoredData, String)> {
    let local_storage = get_local_storage().unwrap();

    match local_storage.get(STORAGE_NAME).unwrap() {
        None => None,
        Some(json_str) => {
            let data:StoredData = serde_json::from_str(&json_str).map_err(|_| JsValue::from_str("couldn't deserialize storage")).unwrap();
            Some((data, json_str))
        }
    }
}

fn save_stored_data(data:&StoredData) -> Result<(), JsValue> {
    let local_storage = get_local_storage()?;

    let json_str = serde_json::to_string(&data).map_err(|_| JsValue::from_str("couldn't serialize storage"))?;

    local_storage.set(STORAGE_NAME, &json_str)
}


fn get_local_storage() -> Result<Storage, JsValue> {
    window().unwrap()
        .local_storage()?
        .ok_or(JsValue::from_str("could not get local storage!"))
}