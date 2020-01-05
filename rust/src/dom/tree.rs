use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt, MutableVec};
use dominator::{Dom, class, html, clone, events, with_node, DomBuilder, apply_methods, make_custom_event_serde};
use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
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
                .property_signal("items", to_array(items_all()))
            }),
            html!("todo-footer"),
        ])
    })
}

struct ListItem {
    id: EntityId, 
    label: &'static str, 
    complete: bool
}

fn item_ids() -> impl SignalVec<Item = EntityId> {
    WORLD.run::<Unique<&TodoList>, _, _>(|todo_list| {
        todo_list.0.signal_vec()
    })
}

fn items_all() -> impl SignalVec<Item = ListItem> {
    item_ids().map(|entity_id| {
        WORLD.run::<(Unique<&TodoList>, &ItemLabel, &ItemComplete), _, _>(|(todo_list, labels, completes)| {
            let (label, complete) = (&labels, &completes).get(entity_id).unwrap();
            ListItem {
                id: entity_id,
                label: &label.0,
                complete: complete.0
            }
            //let dom_item = (serde_json::to_string(&entity_id).unwrap(), &label.0, complete.0);
            //let dom_item:JsValue = serde_wasm_bindgen::to_value(&dom_item).unwrap();
            //dom_item
        })
    })
}

fn to_array(input:impl SignalVec<Item = ListItem>) -> js_sys::Array {
    js_sys::Array::new()
} 
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