
// standard libraries
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

// crate utilities
use crate::{
  utils::{
    me_chimeric_pair::MEChimericPair,
  }
};

// modules
mod sv_registry;


pub fn sv_controller (
  directory: &String,
  pair_end_reference_alignment: &String,
  hash_map_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

  // load reference chromosome aligned reads
  // for i in 1..3 {

    // let prefix = sv_aligned_prefix.clone();
    let c_directory = directory.clone();

    let c_hash_map_collection = hash_map_collection.clone();
    let c_hash_map_anchor = hash_map_anchor.clone();

    // let sv_handle = thread::spawn(move || {

      // let sufix = "".to_string();
      // let sv_aligned_file = format!("{}{}{}{}", c_directory, prefix, i, sufix);
      let sv_aligned_file = format!("{}{}", c_directory, pair_end_reference_alignment);
      println!("{}", sv_aligned_file);

        sv_registry::sv_mapper(
          &sv_aligned_file,
          c_hash_map_collection,
          c_hash_map_anchor,
        ).expect(&sv_aligned_file);

    // });
    // sv_handle.join().unwrap();

  // }
  Ok(())
}
