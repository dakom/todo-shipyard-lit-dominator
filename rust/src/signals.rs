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
use crate::dom::{ListItem};

pub fn phase(world:&World) -> impl Signal<Item = PhaseType> {
    world.run::<Unique<&Phase>, _, _>(|phase| {
        phase.0.signal()
    })
}

pub fn filter_js(world:&World) -> impl Signal<Item = JsValue> {
    world.run::<Unique<&Filter>, _, _>(|filter| {
        filter.0.signal()
            .map(|filter| {
                let filter = filter as u32 as f64;
                JsValue::from_f64(filter)
            })
    })
}

#[derive(PartialEq, Copy, Clone)]
pub enum SignalFilterType {
    Default,
    Override(FilterType)
}

pub fn item_ids(world:Rc<World>, filter_type:Option<SignalFilterType>) -> impl SignalVec<Item = EntityId> {
    let list_signal = world.run::<Unique<&List>, _, _>(|list| { 
        list.0.signal_vec()
    });

    list_signal
        .filter_signal_cloned(move |entity_id| {
            let (filter_signal, complete_signal) = world.run::<(Unique<&Filter>, &Complete), _, _>(|(filter, completes)| {
                (
                    filter.0.signal(),
                    (&completes).get(*entity_id).unwrap().0.signal()
                )
            });

            map_ref!(
                let filter = filter_signal, 
                let complete = complete_signal => move {
                    if filter_type.is_none() {
                        true
                    } else {
                        let filter = match filter_type.unwrap() {
                            SignalFilterType::Override(override_filter_type) => {
                                override_filter_type
                            },
                            SignalFilterType::Default => {
                                *filter
                            }
                        };
                        match filter {
                            FilterType::All => true,
                            FilterType::Active => !complete,
                            FilterType::Completed => *complete
                        }
                    }
                }
            )
        })
}

pub fn items_len_js(world:Rc<World>, filter_type:Option<SignalFilterType>) -> impl Signal<Item = JsValue> {
    item_ids(world, filter_type)
        .len()
        .map(|n| JsValue::from_f64(n as f64))
}

pub fn all_items(world:Rc<World>) -> impl SignalVec<Item = ListItem> {
    item_ids(world.clone(), None)
        .map_signal(move |id| {
                item(&world, id)
        })
}

pub fn item_js(world:&World, entity_id:EntityId) -> impl Signal<Item = JsValue> {
    item(world, entity_id)
        .map(|item| item.to_js_value())
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
