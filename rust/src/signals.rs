use futures_signals:: {
    signal::{Mutable, Signal, SignalExt, always},
    signal_vec::{SignalVec, SignalVecExt, MutableVec, MutableSignalVec},
    map_ref
};
use dominator::{Dom, class, html, clone, events, with_node, DomBuilder, apply_methods, make_custom_event_serde};
use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize};
use js_sys::Array;
use crate::components::*;
use std::rc::Rc;

pub fn phase(world:&World) -> impl Signal<Item = PhaseType> {
    world.run::<Unique<&Phase>, _, _>(|phase| {
        phase.0.signal()
    })
}


pub fn item_ids(world:Rc<World>, with_filter:bool) -> impl SignalVec<Item = EntityId> {
    let list_signal = world.run::<Unique<&List>, _, _>(|list| { 
        list.0.signal_vec() 
    });

    list_signal
        .filter(clone!(with_filter => move |entity_id| {
            //filtering needs to run the world at the time of its call
            world.run::<(Unique<&Filter>, &Complete) , _, _>(|(filter, completes)| {
                if with_filter {
                    let complete = (&completes).get(*entity_id).unwrap().0.get();
                    let filter = filter.0.get();
                    match filter {
                        FilterType::All => true,
                        FilterType::Active => !complete,
                        FilterType::Completed => complete
                    }
                } else {
                    true 
                }
            })
        }))
}

pub fn items_len_js(world:Rc<World>, with_filter:bool) -> impl Signal<Item = JsValue> {
    item_ids(world, with_filter)
        .len()
        .map(|n| JsValue::from_f64(n as f64))
}

pub fn all_items(world:Rc<World>) -> impl SignalVec<Item = ListItem> {
    item_ids(world.clone(), false)
        .map_signal(move |id| {
                item(&world, id)
        })
}

pub fn item_js(world:&World, entity_id:EntityId) -> impl Signal<Item = JsValue> {
    item(world, entity_id).map(|item| item.to_js_value())
}

pub fn item(world:&World, entity_id:EntityId) -> impl Signal<Item = ListItem> {

    let (label, complete) = world.run::<(&Label, &Complete), _, _>(|(labels, completes)| {
        let (label, complete) = (&labels, &completes).get(entity_id).unwrap();
        (label.0.signal_cloned(), complete.0.signal())
    });

    map_ref! {
        label,
        complete => move {
            ListItem {
                id: entity_id,
                label: label.to_string(),
                complete: *complete
            }
        }
    }
}

#[derive(Serialize, Clone)]
pub struct ListItem {
    pub id: EntityId, 
    pub label: String, 
    pub complete: bool
}

impl ListItem {
    fn to_js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}