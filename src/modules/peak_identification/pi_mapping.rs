
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
    mobel_counter::MobelCounter,
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

      let mut mobel_counter = MobelCounter::new();

      if let Some(me_read) = hm_collection.lock().unwrap().get(id_read) {

        match &me_read.chranchor {

          ChrAnchor::Read1 => {

            // TODO: refine by oritentation & strand
            // TODO: implement a function
            // TODO: implement a Poisson distribution threshold

            for i in me_read.read2.me_read.iter() {

              if i.orientation == "upstream".to_string() {
                mobel_counter.upstream = mobel_counter.upstream + 1;
              } else if i.orientation == "downstream".to_string() {
                mobel_counter.downstream = mobel_counter.downstream + 1;
              }

              println!("{:?}", i.orientation);

              // match i.orientation {
              //   "upstream".to_string() => {i.orientation = i.orientation + 1;}
              // }

            }
            println!("{:?}", mobel_counter);
            println!("{:?}", me_read.read1.chr_read[0].flag);

            if me_read.read1.chr_read[0].flag == 0 && mobel_counter.upstream > mobel_counter.downstream {
              println!("Annotated");
            // if me_read.read1.chr_read[0].flag == 0 && me_read.read2.me_read[0].orientation == "upstream".to_string() {

            let binned_position = me_read.read1.chr_read[0].binner();

            if ! chr_position_hm.contains_key(&binned_position) {
              chr_position_hm.insert(binned_position, Vec::new());
            }

            if let Some(id_vector) = chr_position_hm.get_mut(&binned_position)
            {
              id_vector.push(id_read);
            }

              // for i in me_read.read2.me_read.iter() {
              //   println!("{:?}", i.orientation);
              // }
              println!();
              // println!("{:?}", me_read);
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
