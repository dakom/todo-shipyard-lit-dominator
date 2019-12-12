use shipyard::*;
use crate::components::*;

#[system(ClearDirty)]
pub fn run (mut dirties:&mut Dirty) {
    let entity_ids:Vec<Key> = 
        (&dirties)
            .iter()
            .with_id()
            .map(|(id, _)| id)
            .collect();

    for id in entity_ids {
        Remove::<(Dirty,)>::remove((&mut dirties,), id);
    }
}