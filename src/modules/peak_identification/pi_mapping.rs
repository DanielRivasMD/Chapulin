
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
  settings::{
    constants::STRAND_VEC
  },
};


pub fn pi_identifier (
  ikey: &String,
  hm_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

  // let str_vec = ["F5","F3", "R5", "R3"];

  // let mut ids_read = vec![];

  let mut chr_position_hm = HashMap::new();

  for strand in STRAND_VEC.iter() {
    println!("{}", strand);

    chr_position_hm.insert(strand, HashMap::new());
    let tmp_position_hm = chr_position_hm.get_mut(strand).unwrap();

    // let mut tmp_position_hm = HashMap::new();
    // let mut chr_position_hm = HashMap::new();

    // TODO: refine by oritentation & strand
    // TODO: tag orientation to reduce elements to iterate on
    // TODO: implement a function
    // TODO: implement a Poisson distribution threshold
    // TODO: check for non-oriented mobels

    let ids_read = an_registry.lock().unwrap().get(ikey).unwrap().clone();
    // if let Some(ids_read) = an_registry.lock().unwrap().get(ikey) {
      for id_read in ids_read {

        let mut mobel_counter = MobelCounter::new();

        if let Some(me_read) = hm_collection.lock().unwrap().get(&id_read) {
          match &me_read.chranchor {
            ChrAnchor::Read1 => {

              match *strand {
                "F5" => {

                  for i in me_read.read2.me_read.iter() { mobel_counter.counter(&i.orientation); }

                  // {
                    // if i.orientation == "upstream".to_string() {
                    //   mobel_counter.upstream = mobel_counter.upstream + 1;
                    // } else if i.orientation == "downstream".to_string() {
                    //   mobel_counter.downstream = mobel_counter.downstream + 1;
                    // }

                  //   println!("{:?}", i.orientation);
                  //
                  // }

                  println!("{:?}", mobel_counter);
                  println!("{:?}", me_read.read1.chr_read[0].flag);

                  if me_read.read1.chr_read[0].flag == 0 && mobel_counter.upstream >= mobel_counter.downstream {
                    println!("Annotated");
                    // if me_read.read1.chr_read[0].flag == 0 && me_read.read2.me_read[0].orientation == "upstream".to_string() {

                    let binned_position = me_read.read1.chr_read[0].binner();

                    if ! tmp_position_hm.contains_key(&binned_position) {
                      tmp_position_hm.insert(binned_position, Vec::new());
                    }

                    if let Some(id_vector) = tmp_position_hm.get_mut(&binned_position)
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
                "F3" => {},
                "R5" => {},
                "R3" => {},
                _ => {},
              }

            },

            ChrAnchor::Read2 => {
              let binned_position = me_read.read2.chr_read[0].binner();

              if ! tmp_position_hm.contains_key(&binned_position) {
                tmp_position_hm.insert(binned_position, Vec::new());
              }

              if let Some(id_vector) = tmp_position_hm.get_mut(&binned_position)
              {
                id_vector.push(id_read);
              }
            },

            ChrAnchor::None => (),
          }
        }
      }

      println!();
      for (chr_pos, id_vec) in tmp_position_hm.iter() {
        if id_vec.len() > 5 {
          println!("Position: {} => {}", chr_pos, id_vec.len());
          println!("IDs: {:?}", id_vec);
        }
      }

    // }
        // chr_position_hm.insert(single_str, tmp_position_hm);
  }
  Ok(println!("{} {}", "Chromosome: ", &ikey))
}
