use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt, MutableVec, MutableSignalVec};
use dominator::{Dom, class, html, clone, events, with_node, DomBuilder, apply_methods, make_custom_event_serde};
use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize};
use js_sys::Array;
use crate::components::*;
use crate::events::{
    event_types::*,
    event_handlers as handle_event
};
use super::signals;
use std::rc::Rc;

macro_rules! html_at_slot {
    ($name:expr, $slot:expr, { $($rest:tt)* }) => {
        html!($name, {
            .attribute("slot", $slot)
            $($rest)*
        })
    };
}

pub fn render(world:Rc<World>) -> Dom {
    html!("div", {
        .child_signal(
            signals::phase(&world).map(clone!(world => move |phase| {
                if phase == PhaseType::Waiting || phase == PhaseType::Loading {
                    Some(loading())
                } else {
                    Some(app(world.clone()))
                }
            }))
        )
    })
}

fn loading() -> Dom {
    html!("h1", { .text("loading...") })
}

fn app(world:Rc<World>) -> Dom {
    html!("todo-app", {
        .children(&mut [
            header(world.clone()),
            main(world.clone()),
            footer(world.clone()),
        ])
    })
}

fn header(world:Rc<World>) -> Dom {
    html!("todo-header", {
        .children(&mut [
            html_at_slot!("todo-input", "input", {
                .event(clone!(world => move |event: AddTodoEvent| {
                    handle_event::add_todo(&world, &event.data().label);
                }))
            })
        ])
    })
}

fn main(world:Rc<World>) -> Dom {
    html!("todo-main", {
        .property_signal("len", signals::items_len_js(world.clone(), true))
        .children_signal_vec(
            signals::item_ids(world.clone(), true)
                .map(clone!(world => move |entity_id| {
                    item(&world, entity_id)
                }))
        )
    })
}

fn item(world:&World, entity_id:EntityId) -> Dom {
    html!("todo-item", {
        .property_signal("item", signals::item_js(world, entity_id))
    })
}
fn footer(world:Rc<World>) -> Dom {
    html!("todo-footer")
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