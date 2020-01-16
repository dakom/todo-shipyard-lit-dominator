use dominator::{make_custom_event_serde};
use dominator::traits::{StaticEvent};
use web_sys::{EventTarget, CustomEvent};
use wasm_bindgen::{JsValue, JsCast};
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
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
                            let expected = serde_json::to_string(&$data::default()).unwrap();
                            let got = serde_json::to_string(&data).unwrap();
                            if expected != got {
                                Err(JsValue::from_str(&format!("did not match default! should be {} but is {}", expected, got)))
                            } else {
                                Ok(JsValue::from_str(&got))
                            }
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
#[cfg_attr(feature = "ts_test", derive(Default, Serialize))]
#[derive(Deserialize)]
pub struct ClearCompleted;
make_event!("clear-completed",ClearCompleted);

// Reposition 
#[cfg_attr(feature = "ts_test", derive(Serialize))]
#[derive(Deserialize)]
pub struct Reposition {
    pub src: EntityId,
    pub dest: EntityId,
    pub side: DropSide
}

#[cfg_attr(feature = "ts_test", derive(Serialize_repr))]
#[derive(Deserialize_repr, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum DropSide {
    None,
    Bottom,
    Top,
}

make_event!("reposition-list",Reposition);

//impl defaults for tests
cfg_if! {
    if #[cfg(feature = "ts_test")] {
        impl Default for AddTodo {
            fn default() -> Self {
                Self {
                    label: "hello".to_string() 
                }
            }
        }

        impl Default for ToggleTodo {
            fn default() -> Self {
                Self {
                    id: serde_json::from_str("[1,0]").unwrap(),
                    complete: true 
                }
            }
        }

        impl Default for ToggleAllTodos {
            fn default() -> Self {
                Self {
                    complete: true 
                }
            }
        }
        impl Default for ChangeTodo {
            fn default() -> Self {
                Self {
                    id: serde_json::from_str("[1,0]").unwrap(),
                    label: "hello".to_string()
                }
            }
        }

        impl Default for RemoveTodo {
            fn default() -> Self {
                Self {
                    id: serde_json::from_str("[1,0]").unwrap(),
                }
            }
        }

        impl Default for Reposition {
            fn default() -> Self {
                Self {
                    src: serde_json::from_str("[1,0]").unwrap(),
                    dest: serde_json::from_str("[2,0]").unwrap(),
                    side: DropSide::Top
                }
            }
        }
    }
}