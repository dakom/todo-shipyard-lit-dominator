use shipyard::prelude::*;
use crate::components::*;
use crate::dom::{self, effects, effects::DomItem};

//runs when: 
// 1. list of items has changed via adding/removing
// 2. filter changed
#[system(RenderList)]
pub fn run (item_lists:&ItemList, filter:Unique<&'a Filter>, item_labels:&ItemLabel, item_completes:&ItemComplete, dirty_tags:&DirtyTag, dirty_filter: Unique<&'a DirtyFilter>) {
    if (item_lists, &dirty_tags).iter().next().is_some() || dirty_filter.0 {
    //if dirty_tags.iter().next().is_some() || dirty_filter.0 {
        let items:Vec<DomItem> = 

            //TODO - use proper key id. See https://github.com/leudz/shipyard/issues/23
            /*(item_labels, item_completes).iter().with_id()
                .map(|(id, label, complete)| (format!("{}", id.0), label.0.as_ref(), complete.0))
                */
            (item_labels, item_completes).iter().enumerate()
                .map(|(index, (label, complete))| (index.to_string(), label.0.as_ref(), complete.0))
                .filter(|(_, _, complete):&(String, &str, bool)| 
                    match filter {
                        Filter::All => true,
                        Filter::Active => !*complete,
                        Filter::Completed => *complete
                    }
                )
                .map(|(id, label, complete)| (id, label, complete).into())
                .collect();

        effects::set_items(items).unwrap();
    }
}

//runs when item status changes or items are added/removed
#[system(RenderCompleted)]
pub fn run (item_lists: &ItemList, item_completes:&ItemComplete, dirty_tags:&DirtyTag) {
    if(item_lists, &dirty_tags).iter().next().is_some() 
    || (&item_completes, &dirty_tags).iter().next().is_some() { 
        let total_len = (&item_completes).iter().count();
        let completed_len = (&item_completes).iter().filtered(|complete| complete.0).count();
        let remaining_len = total_len - completed_len;

        effects::set_count(total_len, completed_len, remaining_len).unwrap();
    }
}

//runs only when the filter has changed
#[system(RenderFilter)]
pub fn run (filter:Unique<&'a Filter>, dirty_filter:Unique<&'a DirtyFilter>) {
    if dirty_filter.0 {
        effects::set_filter(*filter).unwrap();
    }
}

//runs when any of the following are true
//1. item text has changed
//2. item status has changed 
#[system(RenderContents)]
pub fn run (item_labels:&ItemLabel, item_completes:&ItemComplete, dirty_tags:&DirtyTag) {
    //TODO - use proper key id. See https://github.com/leudz/shipyard/issues/23
    for (index, (item_complete, _)) in (item_completes, dirty_tags).iter().enumerate() {
        let id:String = index.to_string(); 
        effects::set_completed(&id, item_complete.0).unwrap();
    }
}

