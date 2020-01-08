use shipyard::prelude::*;
use crate::components::*;
use crate::dom::storage::*;
use crate::helpers;

#[system(Save)]
pub fn run (phase:Unique<&'a mut Phase>, labels:&Label, completes:&Complete) {
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
#[system(Load)]
pub fn run (mut entities:EntitiesMut, list:Unique<&'a List>, mut labels:&mut Label, mut completes:&mut Complete) {
    if let Some((data, data_str)) = load_stored_data() {
        log::info!("got saved data - creating loaded session!");
        data.items.iter().for_each(|(label, completed)| {
            helpers::add_todo((&mut entities, &mut labels, &mut completes, &list.0), label, *completed);
        });
    } else {
        log::info!("no saved data - new session!");
    }
}

