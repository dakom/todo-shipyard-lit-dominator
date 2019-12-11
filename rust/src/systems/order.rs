use shipyard::*;
use crate::components::*;
use crate::dom;


#[system(Reorder)]
pub fn run (item_lists:&crate::components::ItemList, items:&Item, dirties:&Dirty) {
    /*
    //TODO 

    if let Some((_item_list, _dirty)) = (item_lists, dirties).iter().next() {
        let data:Vec<&str> = items.iter().map(|item| item.0.as_ref()).collect();
        dom::items::replace_items(&data).unwrap();
    }
    //See: https://github.com/leudz/shipyard/blob/85acb2b8b39b6e4c252bba3140a5ff1c54d30a62/src/lib.rs#L1217
    */
}