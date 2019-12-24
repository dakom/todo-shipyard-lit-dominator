use shipyard::prelude::*;
use crate::components::*;

#[system(ClearDirty)]
pub fn run (mut dirty_tags:&mut DirtyTag, mut dirty_filter: Unique<&'a mut DirtyFilter>) {
    let entity_ids:Vec<EntityId> = 
        (&dirty_tags)
            .iter()
            .with_id()
            .map(|(id, _)| id)
            .fold(Vec::new(), |mut vec, x| {vec.push(x); vec});

    for id in entity_ids {
        Remove::<(DirtyTag,)>::remove((&mut dirty_tags,), id);
    }

    dirty_filter.0 = false;
}