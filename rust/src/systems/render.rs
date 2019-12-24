use shipyard::prelude::*;
use crate::components::*;
use crate::dom::{self, effects, effects::DomItem};

//runs when: 
// 1. list of items has changed via adding/removing
// 2. filter changed
#[system(RenderList)]
pub fn run (item_lists:&ItemList, filter:Unique<&'a Filter>, item_labels:&ItemLabel, item_completes:&ItemComplete, dirty_tags:&DirtyTag, dirty_filter: Unique<&'a DirtyFilter>) {
    if (item_lists, &dirty_tags).iter().next().is_some() || dirty_filter.0 {
        let items:Vec<DomItem> = 
            (item_labels, item_completes).iter().with_id()
                .map(|(entity, (label, complete))| (entity, (label.0.as_ref(), complete.0)))
                .filter(|(_, (_, complete)):&(EntityId, (&str, bool))| 
                    match filter {
                        Filter::All => true,
                        Filter::Active => !*complete,
                        Filter::Completed => *complete
                    }
                )
                .map(|(id, (label, complete))| (id, (label, complete)).into())
                .fold(Vec::new(), |mut vec, x| {vec.push(x); vec});

        effects::set_items(items).unwrap();
    }
}

//runs when item status changes or items are added/removed
#[system(RenderCompleted)]
pub fn run (item_lists: &ItemList, item_completes:&ItemComplete, dirty_tags:&DirtyTag) {
    if(item_lists, &dirty_tags).iter().next().is_some() 
    || (&item_completes, &dirty_tags).iter().next().is_some() { 
        let total_len = (&item_completes).iter().count();
        let completed_len = (&item_completes).iter().filter(|complete| complete.0).count();
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
    (&item_labels, &item_completes, &dirty_tags).iter().with_id().for_each(|(entity, (label, complete, _))| {
        effects::set_item(entity, &label.0, complete.0).unwrap();
    });
}

//runs when item status has changed or there are 0 items
#[system(RenderTopToggle)]
pub fn run (item_completes:&ItemComplete, dirty_tags:&DirtyTag) {
    if (&item_completes, &dirty_tags).iter().count() > 0 {
        (&item_completes).iter().for_each(|complete| {
            if !complete.0 {
                effects::set_top_toggle(false).unwrap();
                return
            }
        });
        effects::set_top_toggle(true).unwrap();
    } else if (&item_completes).iter().count() == 0 {
        effects::set_top_toggle(false).unwrap();
    }
}