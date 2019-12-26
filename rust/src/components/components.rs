use num_derive::FromPrimitive;    
use std::collections::VecDeque;
use crate::events::Event;

pub struct ItemLabel (pub String);
pub struct ItemComplete(pub bool); 
pub struct ItemList { }

pub struct DirtyTag {}
pub struct DirtyFilter(pub bool);

impl Default for DirtyFilter {
    fn default() -> Self {
        Self(false)
    }
}


#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
pub enum Filter {
    All,
    Active,
    Completed
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}


#[derive(Eq, PartialEq)]
pub enum Phase {
    InitialLoad,
    Ready
}
impl Default for Phase {
    fn default() -> Self {
        Phase::InitialLoad 
    }
}

pub struct EventQueue(pub VecDeque<Event>);
impl Default for EventQueue {
    fn default() -> Self {
        Self (VecDeque::new())
    }
}