use dominator::{make_custom_event_serde};
use dominator::traits::{StaticEvent};
use web_sys::{EventTarget, CustomEvent};
use wasm_bindgen::{JsValue, JsCast};
use serde::{Serialize, Deserialize};
use shipyard::prelude::*;
use crate::components::*;
use std::rc::Rc;

// Add
#[derive(Deserialize)]
pub struct AddTodo {
    pub label: String 
}
make_custom_event_serde!(AddTodoEvent, "add-todo", AddTodo);

// Remove
#[derive(Deserialize)]
pub struct RemoveTodo {
    pub id: EntityId 
}
make_custom_event_serde!(RemoveTodoEvent, "remove-todo", RemoveTodo);

// Toggle
#[derive(Deserialize)]
pub struct ToggleTodo {
    pub id: EntityId,
    pub complete: bool
}
make_custom_event_serde!(ToggleTodoEvent, "toggle-todo", ToggleTodo);

// Change 
#[derive(Deserialize)]
pub struct ChangeTodo {
    pub id: EntityId,
    pub label: String 
}
make_custom_event_serde!(ChangeTodoEvent , "change-todo", ChangeTodo);

// ToggleAll 
#[derive(Deserialize)]
pub struct ToggleAllTodos {
    pub complete: bool
}
make_custom_event_serde!(ToggleAllTodosEvent, "toggle-all-todos", ToggleAllTodos);

/*
use crate::components::Filter;
use num_derive::FromPrimitive;    
use std::convert::TryFrom;
use num_traits::FromPrimitive;
use shipyard::prelude::*;
use std::collections::VecDeque;
use std::sync::Mutex;
use lazy_static::*;

#[cfg(feature = "ts_test")]
use strum_macros::{EnumIter, AsRefStr};

//Events as sent from JS (straight enum)
#[cfg_attr(feature = "ts_test", derive(EnumIter, AsRefStr))]
#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
pub enum BridgeEvent {
    InitialLoad,
    FilterChange,
    AddTodo,
    SetCompleted,
    RemoveTodo,
    ClearCompleted,
    SetCompletedAll,
    ChangeTodo,
}

//Events as we want to deal with them in Rust
pub enum Event {
    InitialLoad,
    FilterChange(Filter),
    AddTodo(String),
    SetCompleted(EntityId, bool),
    SetCompletedAll(bool),
    RemoveTodo(EntityId),
    ClearCompleted,
    ChangeTodo(EntityId, String),
}

impl TryFrom<u32> for BridgeEvent {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("BridgeEvent: {} is outside of range!", value))
    }
}

impl TryFrom<u32> for Filter {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Filter: {} is outside of range!", value))
    }
}
*/