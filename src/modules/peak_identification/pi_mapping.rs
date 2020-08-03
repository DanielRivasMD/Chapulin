
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::{
    me_chimeric_pair::MEChimericPair,
    chr_anchor_enum::ChrAnchorEnum,
    strander::strander,
    thresholder::thresholder,
  },
  settings::{
    constants::{
      STRAND_VEC,
      NO_FDR,
    },
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn pi_identifier (
  ikey: &String,
  hm_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
  chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> std::io::Result<()> {

  let mut chr_position_hm = HashMap::new();
  let chr_size = chr_assembly.lock().unwrap().get(ikey).unwrap().clone();

  for strand in STRAND_VEC.iter() {

    chr_position_hm.insert(strand, HashMap::new());
    let tmp_position_hm = chr_position_hm.get_mut(strand).unwrap();

    // TODO: tag orientation to reduce elements to iterate on
    // TODO: check for non-oriented mobels
    // TODO: implement a threshold selector

    let mut read_count = 0;

    let ids_read = an_registry.lock().unwrap().get(ikey).unwrap().clone();

    for id_read in ids_read {

      if let Some(me_read) = hm_collection.lock().unwrap().get(&id_read) {
        match &me_read.chranch {
          ChrAnchorEnum::Read1 => {
            read_count = strander(
              id_read,
              strand,
              read_count,
              &me_read.read1.chr_read[0],
              &me_read.read2.me_read,
              tmp_position_hm
            );
          },

          ChrAnchorEnum::Read2 => {
            read_count = strander(
              id_read,
              strand,
              read_count,
              &me_read.read2.chr_read[0],
              &me_read.read1.me_read,
              tmp_position_hm
            );
          },

          ChrAnchorEnum::None => (),
        }
      }
    }

    if ! ( read_count == 0 ) {
      let pois_threshold = thresholder(
        read_count as f64,
        chr_size,
        0.001,
        tmp_position_hm,
        NO_FDR,
      );

      for (chr_pos, id_vec) in tmp_position_hm.iter() {
        if id_vec.len() > pois_threshold {
          println!("Position: {} @ strand: {} => {}", chr_pos, strand, id_vec.len());
          println!("IDs: {:?}", id_vec);
        }
      }
    }

  }
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
