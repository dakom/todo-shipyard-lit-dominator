use num_derive::FromPrimitive;    
use num_traits::FromPrimitive;
//Events as we want to deal with them in Rust
pub enum Event {
    ChangePage(Page),
    AddTodo(String),
    UpdateTodo(shipyard::Key, String),
    RemoveTodo(shipyard::Key)
}

#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
pub enum Page {
    Init,
}