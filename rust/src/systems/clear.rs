use shipyard::prelude::*;
use crate::components::*;
use crate::dom::storage::*;

#[system(ClearUpdated)]
pub fn run (mut item_labels:&mut ItemLabel, mut item_completes:&mut ItemComplete) {
    item_labels.clear_modified();
    item_labels.clear_inserted();
    item_completes.clear_modified();
    item_completes.clear_inserted();
}