use shipyard::prelude::*;
use super::event_types::*;
use crate::components::*;
use crate::actions;

pub fn add_todo(world:&World, label:&str) {
    world.run::<((EntitiesMut, &mut Label, &mut Complete), Unique<&List>), _, _>(|(mut views, list)| {
        actions::add_todo((&mut views, &list.0), label, false);
    });
}