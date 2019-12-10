use shipyard::*;
use crate::events;

pub struct Item(pub String);
pub struct Order(pub Vec<Key>);

pub struct Dirty {}