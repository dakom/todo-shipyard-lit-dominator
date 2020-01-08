use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures_signals::signal_vec::{SignalVec, SignalVecExt, MutableVec, MutableSignalVec};
use shipyard::prelude::*;
use shipyard::internal::{EntitiesViewMut, ViewMut};
use crate::components::*;

pub fn add_todo(storages:(&mut (EntitiesViewMut, ViewMut<Label>, ViewMut<Complete>), &MutableVec<EntityId>), label:&str, complete: bool) {
    let ((entities, labels, completes), todo_list) = storages;

    let entity = entities.add_entity((labels, completes), 
        (
            Label(Mutable::new(label.to_string())), 
            Complete(Mutable::new(false))
        ));

    todo_list.lock_mut().push(entity);

    log::info!("added entity {:?} with label {}", entity, label);
}
