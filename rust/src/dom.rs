use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt, MutableVec, MutableSignalVec};
use dominator::{Dom, class, html, clone, events, with_node, DomBuilder, apply_methods, make_custom_event_serde};
use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize};
use js_sys::Array;
use crate::components::*;
use crate::actions;
use crate::events::*;
use super::signals::{self, SignalFilterType};
use std::rc::Rc;

macro_rules! html_at_slot {
    ($name:expr, $slot:expr, { $($rest:tt)* }) => {
        html!($name, {
            .attribute("slot", $slot)
            $($rest)*
        })
    };
}

#[derive(Serialize, Clone)]
pub struct ListItem {
    pub id: EntityId, 
    pub label: String, 
    pub complete: bool
}

impl ListItem {
    pub fn to_js_value(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self).unwrap()
    }
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
                    actions::add_todo(&world, &event.data().label, false);
                }))
            })
        ])
    })
}

fn main(world:Rc<World>) -> Dom {
    html!("todo-main", {
        .property_signal("len", signals::items_len_js(world.clone(), None))
        .children_signal_vec(
            signals::item_ids(world.clone(), Some(SignalFilterType::Default))
                .map(clone!(world => move |entity_id| {
                    item(world.clone(), entity_id)
                }))
        )
        .event(clone!(world => move |event:ToggleAllTodosEvent| {
            actions::toggle_all_todos(&world, event.data().complete);
        }))
    })
}

fn item(world:Rc<World>, entity_id:EntityId) -> Dom {
    html!("todo-item", {
        .property_signal("item", signals::item_js(&world, entity_id))
        .event(clone!(world => move |event:RemoveTodoEvent| {
            actions::remove_todo(&world, event.data().id);
        }))
        .event(clone!(world => move |event:ToggleTodoEvent| {
            actions::toggle_todo(&world, event.data().id, event.data().complete);
        }))
        .event(clone!(world => move |event:ChangeTodoEvent| {
            actions::change_todo(&world, event.data().id, &event.data().label);
        }))
    })
}
fn footer(world:Rc<World>) -> Dom {
    html!("todo-footer", {
        .property_signal("total", signals::items_len_js(world.clone(), None))
        .property_signal("remaining", signals::items_len_js(world.clone(), Some(SignalFilterType::Override(FilterType::Active))))
        .property_signal("completed", signals::items_len_js(world.clone(), Some(SignalFilterType::Override(FilterType::Completed))))
        .property_signal("filter", signals::filter_js(&world))
        .event(clone!(world => move |event:ClearCompletedEvent| {
            actions::clear_completed(&world);
        }))
    })
}

/*
    @property( { type : Number} ) total = 0; 
    @property( { type : Number} ) remaining = 0; 
    @property( { type : Number} ) completed = 0; 
    @property( { type : Number} ) filter = Filter.All 
*/