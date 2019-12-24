use crate::components::Filter;
use num_derive::FromPrimitive;    
use std::convert::TryFrom;
use num_traits::FromPrimitive;
use shipyard::prelude::*;
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