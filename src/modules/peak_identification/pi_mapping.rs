
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
  settings::{
    constants::{
      BIN_OVERLAP,
      BIN_SIZE
    }
  }
};

pub fn pi_identifier (
  ikey: &String,
  hm_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
  chr_max: Arc<Mutex<HashMap<String, i32>>>,
) -> std::io::Result<()> {

  // println!("{:#?}", ikey);

  // println!("Chromosome: {} => {:?}", ikey, chr_max.lock().unwrap().get(ikey));
  let mut binned_hashmap = HashMap::new();
  if let Some(chr_max_num) = chr_max.lock().unwrap().get(ikey) {
    let mut n = 0;
    while n < *chr_max_num {
      n = n + BIN_OVERLAP;

        binned_hashmap.insert(n..n + BIN_SIZE, vec![]);

    // while i < 1_000 {


        // println!("{}: {}", i, (i..i+100).contains(&257));

    }

    // // let y = x.clone();
    // for j in x.iter_mut() {
    //
    //     if j.0.contains(&257) {
    //         j.1.push("UNO");
    //     }
    //     println!("{:?}", j);
    //     println!("{}", j.1.len());
    // }
    //
    // println!("{:?}", x);
    // // }

  }

  println!("Binned vector constructed\n");

  if let Some(ids_read) = an_registry.lock().unwrap().get(ikey) {
  // let ids_read = an_registry.lock().unwrap().get(ikey).unwrap();

    // println!("{:#?}", ids_read);
    for id_read in ids_read {
      // println!("{}", id_read);

      if let Some(me_read) = hm_collection.lock().unwrap().get(id_read) {

        // let mut read_pos = 0;

        // println!("{:#?}", me_read.anchor);
        match &me_read.chranchor {
          ChrAnchor::Read1 => {
            // println!("{:#?}", me_read);
            // println!("{}", me_read.read1.chr_read[0].pos);
            // println!("This read 1");
            // read_pos = me_read.read1.chr_read[0].pos / 100;
            for (irange, ivec) in binned_hashmap.iter_mut() {
              if irange.contains(&me_read.read1.chr_read[0].pos) {
                ivec.push(id_read);
              }
            }
          },
          ChrAnchor::Read2 => {
            // println!("{:#?}", me_read);
            // println!("{}", me_read.read2.chr_read[0].pos);
            // println!("This read 2");
            // read_pos = me_read.read2.chr_read[0].pos / 100;
            for (irange, ivec) in binned_hashmap.iter_mut() {
              if irange.contains(&me_read.read1.chr_read[0].pos) {
                ivec.push(id_read);
              }
            }
          },
          ChrAnchor::None => (),
        }

        // println!("Id: {}\tRead: {:?}\tposition: {}", id_read, me_read.chranchor, read_pos);

        // me_read.read1.chr_read[0].pos
        // println!("{:#?}", me_read);


          // let x = me_read.chr_anchor_retriever();
          // let x = me_read.chr_anchor_retriever();
        // println!("{:#?}", &me_read.chr_anchor_retriever());
        // println!("{:#?}", me_read);

        // println!("{:#?}", me_read.chr_anchor_retriever());

        // let x = me_read;
        // match x.anchor {
        //   Anchor::Read1 => {
        //     println!("{:#?}: {:#?}", x.read1, x)
        //   },
        //   Anchor::Read2 => {
        //     println!("{:#?}: {:#?}", x.read2, x)
        //   },
        //   Anchor::ReadDefault => {
        //     println!("This a default value:\n{:#?}", x)
        //   }
        // }

        // let y = x.chr_anchor_retriever();
        // println!("{:#?}", y);
      }
    }
  }

  // for (irange, ivec) in binned_hashmap {
  //   if ivec.len() > 0 {
  //     println!("{:?} => {:?}", irange, ivec);
  //   }
  // }

  // println!("{:#?}", an_registry.lock().unwrap().contains_key(ikey));

  // match anchor_reads {
  //   Some(x) => {
  //     println!("{:#?}", x);
  //   },
  //   None => {
  //     println!("Not found!");
  //   },
  // }



  // Ok(print!(""))
  Ok(println!("{} {}", "Chromosome: ", &ikey))
}
