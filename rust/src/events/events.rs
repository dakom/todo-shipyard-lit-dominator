use crate::components::Filter;
use num_derive::FromPrimitive;    
use std::convert::TryFrom;
use num_traits::FromPrimitive;
#[cfg(feature = "ts_test")]
use strum_macros::{EnumIter, AsRefStr};

//Events as sent from JS (straight enum)
#[cfg_attr(feature = "ts_test", derive(EnumIter, AsRefStr))]
#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
pub enum BridgeEvent {
    FilterChange,
    AddTodo,
    SetTodoCompleted,
    RemoveTodo,
}

impl TryFrom<u32> for BridgeEvent {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("BridgeEvent: {} is outside of range!", value))
    }
}

//Events as we want to deal with them in Rust
pub enum Event {
    FilterChange(Filter),
    AddTodo(String),
    //TODO - use proper key id. See https://github.com/leudz/shipyard/issues/23
    //SetTodoCompleted(shipyard::Key, bool),
    //RemoveTodo(shipyard::Key)
    SetTodoCompleted(usize, bool),
    RemoveTodo(usize)
}

impl TryFrom<u32> for Filter {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FromPrimitive::from_u32(value).ok_or_else(|| format!("Filter: {} is outside of range!", value))
    }
}