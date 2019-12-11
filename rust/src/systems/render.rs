use shipyard::*;
use crate::components::*;
use crate::dom;
use wasm_bindgen::prelude::*;

//TODO - use the filter to cull list
#[system(RenderItemList)]
pub fn run (item_lists:&ItemList, filters:&Filter, items:&Item, dirties:&Dirty) {
    if let Some((_item_list, _dirty)) = (item_lists, dirties).iter().next() {
        let items:Vec<&Item> = items.iter().collect();
        let items_js = serde_wasm_bindgen::to_value(&items).unwrap();
        dom::todo::replace_items(items_js).unwrap();
    }
}

#[system(RenderFilter)]
pub fn run (_filters:&Filter, _dirties:&Dirty) {
}

#[system(RenderItemsUpdate)]
pub fn run (_items:&Item, _dirty:&Dirty) {
    //let data:Vec<&str> = items.iter().map(|x| x.0.as_ref()).collect();
    /*
    for item in items.iter() {
        //log::info!("in system! {}", item.0)
    }
    */
}

#[system(RenderClearDirty)]
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