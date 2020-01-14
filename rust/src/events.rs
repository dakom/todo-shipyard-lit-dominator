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


// ClearCompleted 
#[derive(Deserialize)]
pub struct ClearCompleted;
make_custom_event_serde!(ClearCompletedEvent, "clear-completed", ClearCompleted);