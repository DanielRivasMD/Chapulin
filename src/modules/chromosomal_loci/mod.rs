
// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::{thread};

// modules
mod cl_aligned;

// crate utilities
use crate::{
  utils::{
    me_chimeric_pair::MEChimericPair,
  }
};


pub fn cl_controller (
  directory: &String,
  cl_aligned_prefix: &String,
  hash_map_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

  // load reference chromosome aligned reads
  for i in 1..3 {

    let prefix = cl_aligned_prefix.clone();
    let c_directory = directory.clone();

    let c_hash_map_collection = hash_map_collection.clone();
    let c_hash_map_anchor = hash_map_anchor.clone();

    let cl_handle = thread::spawn(move || {

      let sufix = ".sorted.sam".to_string();
      let cl_aligned_file = format!("{}{}{}{}", c_directory, prefix, i, sufix);

        cl_aligned::cl_mapper(
          &cl_aligned_file,
          c_hash_map_collection,
          c_hash_map_anchor,
        ).unwrap();

    });
    cl_handle.join().unwrap();
  }

  Ok(())
}
