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
make_custom_event_serde!("add-todo", AddTodoEvent, AddTodo);

// Remove
#[derive(Deserialize)]
pub struct RemoveTodo {
    pub id: EntityId 
}
make_custom_event_serde!("remove-todo", RemoveTodoEvent, RemoveTodo);

// Toggle
#[derive(Deserialize)]
pub struct ToggleTodo {
    pub id: EntityId,
    pub complete: bool
}
make_custom_event_serde!("toggle-todo",ToggleTodoEvent, ToggleTodo);

// Change 
#[derive(Deserialize)]
pub struct ChangeTodo {
    pub id: EntityId,
    pub label: String 
}
make_custom_event_serde!("change-todo",ChangeTodoEvent ,  ChangeTodo);

// ToggleAll 
#[derive(Deserialize)]
pub struct ToggleAllTodos {
    pub complete: bool
}
make_custom_event_serde!("toggle-all-todos",ToggleAllTodosEvent,  ToggleAllTodos);


// ClearCompleted 
#[derive(Deserialize)]
pub struct ClearCompleted;
make_custom_event_serde!("clear-completed",ClearCompletedEvent,  ClearCompleted);