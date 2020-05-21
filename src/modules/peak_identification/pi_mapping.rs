
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// crate utilities
use crate::{
  utils::{
    read_record::ReadRecord,
    chranchor_enum::ChrAnchor,
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


pub fn pi_identifier (
  ikey: &String,
  hm_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

  let mut chr_position_hm = HashMap::new();

  for strand in STRAND_VEC.iter() {

    chr_position_hm.insert(strand, HashMap::new());
    let tmp_position_hm = chr_position_hm.get_mut(strand).unwrap();

    // TODO: tag orientation to reduce elements to iterate on
    // TODO: check for non-oriented mobels
    // TODO: implement a threshold selector
    // TODO: write Poisson as an independent module

    let mut read_count = 0;

    let ids_read = an_registry.lock().unwrap().get(ikey).unwrap().clone();
    // if let Some(ids_read) = an_registry.lock().unwrap().get(ikey) {

    for id_read in ids_read {

      if let Some(me_read) = hm_collection.lock().unwrap().get(&id_read) {
        match &me_read.chranchor {
          ChrAnchor::Read1 => {
            read_count = strander(id_read, strand, read_count, &me_read.read1.chr_read[0], &me_read.read2.me_read, tmp_position_hm);
          },

          ChrAnchor::Read2 => {
            read_count = strander(id_read, strand, read_count, &me_read.read2.chr_read[0], &me_read.read1.me_read, tmp_position_hm);
          },

          ChrAnchor::None => (),
        }
      }
    }

    println!();
    let pois_threshold = thresholder(read_count, 1_000_000, 0.001, tmp_position_hm, NO_FDR);
    for (chr_pos, id_vec) in tmp_position_hm.iter() {
      if id_vec.len() > pois_threshold as usize {
        println!("Position: {} => {}", chr_pos, id_vec.len());
        println!("IDs: {:?}", id_vec);
      }
    }

  }
  Ok(println!("{} {}", "Chromosome: ", &ikey))
}
