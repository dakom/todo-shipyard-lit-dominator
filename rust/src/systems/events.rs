use shipyard::prelude::*;
use crate::components::*;
use crate::events::*;

#[system(Add)]
pub fn run (event_queue:Unique<&'a mut EventQueue>, entities:&mut EntitiesMut, item_labels:&mut ItemLabel, item_completes:&mut ItemComplete, dirty_tags:&DirtyTag) {
    let next_event = event_queue.0[0];
    if let Event::AddTodo(label) = next_event {
        let entity = entities.add_entity((&mut item_labels, &mut item_completes), (ItemLabel(label.to_string()), ItemComplete(false)));
        event_queue.0.pop_front();
    }
}