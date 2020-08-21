
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::functions::{
    thresholder::thresholder,
    chr_counter::chr_counter,
  },
  utils::structures::{
    sv_chimeric_pair::SVChimericPair,
  },
  settings::{
    constants::{
      NO_FDR,
    },
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
// use crate::error::{
// }

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn pi_sv_identifier (
  ikey: &str,
  hm_collection: Arc<Mutex<HashMap<String, SVChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
  chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {

  let chr_size = *chr_assembly
    .lock().unwrap()
    .get(ikey).unwrap();
  let ids_read = an_registry
    .lock().unwrap()
    .get(ikey).unwrap()
    .clone();


  let read_count = ids_read.len();

  let mut chr_position_hm = HashMap::new();


  for id_read in ids_read {
    if let Some(sv_pair) = hm_collection
      .lock().unwrap()
      .get(&id_read) {
      chr_counter!(
        id_read,
        sv_pair,
        &mut chr_position_hm
      );
    }
  }

// TODO: memotization
  if read_count != 0 {
    let pois_threshold = thresholder(
      read_count as f64,
      chr_size,
      0.001,
      &chr_position_hm,
      NO_FDR,
    );

    for (chr_pos, id_vec) in chr_position_hm.iter() {

      if id_vec.len() > pois_threshold {
        println!("{}, {}, {}", ikey, chr_pos, id_vec.len());
      }

    }

  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
