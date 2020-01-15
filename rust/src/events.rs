use dominator::{make_custom_event_serde};
use dominator::traits::{StaticEvent};
use web_sys::{EventTarget, CustomEvent};
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use shipyard::prelude::*;
use std::rc::Rc;
use cfg_if::cfg_if;

/// make_event! takes a literal and an ident and does the following:
/// 1. impls what's needed for dominator
/// 2. calls make_custom_event_serde! to enable the .data() helper (also for dominator)
/// 3. creates a function for testing the round-tripping from typescript (when ts_test feature is enabled)

macro_rules! make_event {
    ($literal:literal, $data:ident) => {
        paste::item! {
            make_custom_event_serde!($literal, [<$data Event>], $data);
            cfg_if! {
                if #[cfg(feature = "ts_test")] {
                    #[wasm_bindgen]
                    pub fn [<check_rust_event_ $data>](event:web_sys::CustomEvent) -> Result<JsValue, JsValue> {
                        let literal = event.type_();
                        let event:[<$data Event>] = unsafe {
                            std::mem::transmute::<web_sys::CustomEvent, [<$data Event>]>(event)
                        };

                        if literal == $literal {
                            let data:$data = event.data(); 
                            Ok(JsValue::from_str(&serde_json::to_string(&data).unwrap()))
                        } else {
                            Err(JsValue::from_str(&format!("wrong type! should be {} but is {}", $literal, literal)))
                        }
                    }
                }
            }
        }
    }
}

// Add
#[cfg_attr(feature = "ts_test", derive(Serialize))]
#[derive(Deserialize)]
pub struct AddTodo {
    pub label: String 
}
make_event!("add-todo", AddTodo);

// Remove
#[cfg_attr(feature = "ts_test", derive(Serialize))]
#[derive(Deserialize)]
pub struct RemoveTodo {
    pub id: EntityId 
}
make_event!("remove-todo", RemoveTodo);

// Toggle
#[cfg_attr(feature = "ts_test", derive(Serialize))]
#[derive(Deserialize)]
pub struct ToggleTodo {
    pub id: EntityId,
    pub complete: bool
}
make_event!("toggle-todo", ToggleTodo);

// Change 
#[cfg_attr(feature = "ts_test", derive(Serialize))]
#[derive(Deserialize)]
pub struct ChangeTodo {
    pub id: EntityId,
    pub label: String 
}
make_event!("change-todo",ChangeTodo);

// ToggleAll 
#[cfg_attr(feature = "ts_test", derive(Serialize))]
#[derive(Deserialize)]
pub struct ToggleAllTodos {
    pub complete: bool
}
make_event!("toggle-all-todos",ToggleAllTodos);


// ClearCompleted 
#[cfg_attr(feature = "ts_test", derive(Serialize))]
#[derive(Deserialize)]
pub struct ClearCompleted;
make_event!("clear-completed",ClearCompleted);
