use shipyard::*;
use crate::components::*;
use crate::dom::{self, effects, effects::DomItem};

//runs when: 
// 1. list of items has changed via adding/removing
// 2. filter changed
#[system(RenderList)]
pub fn run (item_lists:&ItemList, filters:&Filter, item_labels:&ItemLabel, item_statuses:&ItemStatus, dirties:&Dirty) {
    if (item_lists, &dirties).iter().next().is_some()
        || (filters, &dirties).iter().next().is_some() {
        let items:Vec<DomItem> = 
            item_labels.iter().zip(item_statuses.iter())
                .map(|(label, status)| (label.0.as_ref(), status.complete).into())
                .collect();

        effects::set_items(items).unwrap();
    }
}

//runs only when item status changes
#[system(RenderCompleted)]
pub fn run (item_statuses:&ItemStatus, dirties:&Dirty) {
    if(&item_statuses, &dirties).iter().next().is_some() { 
        let total_len = (&item_statuses).iter().count();
        let completed_len = (&item_statuses).iter().filtered(|status| status.complete).count();
        let remaining_len = total_len - completed_len;

        effects::set_count(total_len, remaining_len).unwrap();
    }
}

//runs only when the filter has changed
#[system(RenderFilter)]
pub fn run (filters:&Filter, dirties:&Dirty) {
    if let Some((filter, _)) = (filters, dirties).iter().next() {
        effects::set_filter(*filter).unwrap();
        log::info!("filter is dirty!");
    }
}

//runs when any of the following are true
//1. item text has changed
//2. item status has changed 
#[system(RenderContents)]
pub fn run (_item_labels:&ItemLabel, _item_statuses:&ItemStatus, _dirty:&Dirty) {
}

