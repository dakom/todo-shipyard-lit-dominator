use shipyard::prelude::*;
use crate::components::*;

#[system(ClearDirty)]
pub fn run (mut dirty_tags:&mut DirtyTag, mut dirty_filter: Unique<&'a mut DirtyFilter>) {
    let entity_ids:Vec<Key> = 
        (&dirty_tags)
            .iter()
            .with_id()
            .map(|(id, _)| id)
            .collect();

    for id in entity_ids {
        Remove::<(DirtyTag,)>::remove((&mut dirty_tags,), id);
    }

    dirty_filter.0 = false;
}