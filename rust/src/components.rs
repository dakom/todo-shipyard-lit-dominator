use num_derive::FromPrimitive;    
use futures_signals::signal::{Mutable};
use futures_signals::signal_vec::{MutableVec};
use shipyard::prelude::*;

pub struct List(pub MutableVec<EntityId>);
pub struct Filter(pub Mutable<FilterType>);
pub struct Phase(pub Mutable<PhaseType>);
pub struct Label (pub Mutable<String>);
pub struct Complete(pub Mutable<bool>); 
pub struct SaveTag(pub bool);

#[derive(FromPrimitive, PartialEq, Copy, Clone, Debug)]
#[repr(u32)]
pub enum FilterType {
    All,
    Active,
    Completed
}

impl Default for FilterType {
    fn default() -> Self {
        FilterType::All
    }
}


#[derive(Eq, Copy, Clone, PartialEq)]
pub enum PhaseType {
    Waiting,
    Loading,
    Ready
}
impl Default for PhaseType {
    fn default() -> Self {
        PhaseType::Waiting
    }
}


