use shipyard::prelude::*;
use crate::components::*;
use crate::dom::storage::*;

#[system(Save)]
pub fn run (phase:Unique<&'a mut Phase>, item_labels:&ItemLabel, item_completes:&ItemComplete) {
    //TODO
    /*
    // phase is gated so that we don't save the initial empty data 
    if *phase == Phase::Ready {
        if (&item_lists, &dirty_tags).iter().next().is_some() 
            || (&item_completes, &dirty_tags).iter().next().is_some()
            || (&item_labels, &dirty_tags).iter().next().is_some()
            {
                let items:Vec<(&str, bool)> = 
                    (&item_labels, &item_completes)
                        .iter()
                        .map(|tuple| ((tuple.0).0.as_ref(), (tuple.1).0))
                        .fold(Vec::new(), |mut v, x| { v.push(x); v });

                save_stored_data(&StoredData::new(items)).unwrap();
        }
    } else {
        *phase = Phase::Ready;
    }
    */
}