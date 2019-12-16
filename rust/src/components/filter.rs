use num_derive::FromPrimitive;    
use std::fmt;

#[derive(FromPrimitive, Copy, Clone, Debug)]
#[repr(u32)]
pub enum Filter {
    All,
    Active,
    Completed
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}