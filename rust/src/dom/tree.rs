use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt, MutableVec, MutableSignalVec};
use dominator::{Dom, class, html, clone, events, with_node, DomBuilder, apply_methods, make_custom_event_serde};
use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize};
use js_sys::Array;
use crate::components::*;
use crate::world::WORLD;
use crate::events::*;

macro_rules! html_at_slot {
    ($name:expr, $slot:expr, { $($rest:tt)* }) => {
        html!($name, {
            .attribute("slot", $slot)
            $($rest)*
        })
    };
}

pub fn get_dom_tree() -> Dom {
    html!("todo-app", {
        .children(&mut [
            html!("todo-header", {
                .children(&mut [
                    html_at_slot!("todo-input", "input", {
                        .event(|event: AddTodoEvent| {
                            add_todo(&event.data());
                        })
                    })
                ])
            }),
            html!("todo-main", {
                .property_signal("len", items_len_js(true))
                .children_signal_vec(items_js(true).map(|item| {
                    html!("todo-item", {
                        .property("item", item)
                    })
                }))
            }),
            html!("todo-footer"),
        ])
    })
}

#[derive(Serialize)]
struct ListItem <'a> {
    id: EntityId, 
    label: &'a str, 
    complete: bool
}

impl <'a> ListItem <'a> {
    fn to_js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
}

fn item_ids(with_filter:bool) -> impl SignalVec<Item = EntityId> {
    let todo_list_signal = WORLD.run::<Unique<&TodoList>, _, _>(|todo_list| { 
        todo_list.0.signal_vec() 
    });

    todo_list_signal
        .filter(clone!(with_filter => move |entity_id| {
            //filtering needs to run the world at the time of its call
            WORLD.run::<(Unique<&Filter>, &ItemComplete) , _, _>(|(filter, completes)| {
                if with_filter {
                    let complete = (&completes).get(*entity_id).unwrap().0;
                    match filter {
                        Filter::All => true,
                        Filter::Active => !complete,
                        Filter::Completed => complete
                    }
                } else {
                    false
                }
            })
        }))
}

fn items_len_js(with_filter:bool) -> impl Signal<Item = JsValue> {
    item_ids(with_filter)
        .len()
        .map(|n| JsValue::from_f64(n as f64))
}

fn items_js(with_filter:bool) -> impl SignalVec<Item = JsValue> {
    item_ids(with_filter).map(|entity_id| {
        WORLD.run::<(Unique<&TodoList>, &ItemLabel, &ItemComplete), _, _>(|(todo_list, labels, completes)| {
            let (label, complete) = (&labels, &completes).get(entity_id).unwrap();
            let item = ListItem {
                id: entity_id,
                label: label.0.as_ref(),
                complete: complete.0
            };

            log::info!("{:?}", entity_id);
            item.to_js_value()
            //let dom_item = (serde_json::to_string(&entity_id).unwrap(), &label.0, complete.0);
            //let dom_item:JsValue = serde_wasm_bindgen::to_value(&dom_item).unwrap();
            //dom_item
        })
    })
}
/*
fn js_items_all() -> impl Signal<Item = Array> {
    items_all()
        .to_signal_map(|items| {
            items.into_iter().map(|item| item.to_js_value()).collect()
        })
}
*/
/*
.property_signal("items", current_items_all())

fn current_item_ids() -> impl SignalVec<Item = EntityId> { ... }

fn current_items_all() -> impl SignalVec<Item = JsValue> {
    current_item_ids().map(|entity_id| {
        let foo:JsValue = convert_entity(entity_id);
        foo
    })
}
*/