use crate::components::Filter;
use num_derive::FromPrimitive;    

#[cfg(feature = "ts_test")]
use strum_macros::{EnumIter, AsRefStr};

//Events as sent from JS (straight enum)
#[cfg_attr(feature = "ts_test", derive(EnumIter, AsRefStr))]
#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
pub enum BridgeEvent {
    FilterChange,
    AddTodo,
    UpdateTodo,
    RemoveTodo,
}

//Events as we want to deal with them in Rust
pub enum Event {
    FilterChange(Filter),
    AddTodo(String),
    UpdateTodo(shipyard::Key, String),
    RemoveTodo(shipyard::Key)
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
pub enum Page {
    Init,
}