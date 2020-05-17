
// standard libraries
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};

// crate utilities
use crate::{
  utils::{
    read_record::ReadRecord,
    chranchor_enum::ChrAnchor,
  },
};

pub fn pi_identifier (
  ikey: &String,
  hm_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

  let mut chr_position_hm = HashMap::new();

  if let Some(ids_read) = an_registry.lock().unwrap().get(ikey) {

    for id_read in ids_read {

      if let Some(me_read) = hm_collection.lock().unwrap().get(id_read) {

        match &me_read.chranchor {

          ChrAnchor::Read1 => {

            // TODO: refine by oritentation & strand
            // TODO: implement a function
            // TODO: implement a Poisson distribution threshold

            let binned_position = me_read.read1.chr_read[0].binner();

            if ! chr_position_hm.contains_key(&binned_position) {
              chr_position_hm.insert(binned_position, Vec::new());
            }

            if let Some(id_vector) = chr_position_hm.get_mut(&binned_position)
            {
              id_vector.push(id_read);
            }

          },

          ChrAnchor::Read2 => {

            let binned_position = me_read.read2.chr_read[0].binner();

            if ! chr_position_hm.contains_key(&binned_position) {
              chr_position_hm.insert(binned_position, Vec::new());
            }

            if let Some(id_vector) = chr_position_hm.get_mut(&binned_position)
            {
              id_vector.push(id_read);
            }

          },

          ChrAnchor::None => (),

        }
      }
    }

    // println!();
    // for (chr_pos, id_vec) in chr_position_hm.iter() {
    //   if id_vec.len() > 5 {
    //     println!("Position: {} => {}", chr_pos, id_vec.len());
    //     println!("IDs: {:?}", id_vec);
    //   }
    // }

  }

  Ok(println!("{} {}", "Chromosome: ", &ikey))
}
