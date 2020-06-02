
// standard libraries
use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use std::thread;

// crate utilities
use crate::{
  utils::{
    read_record::ReadRecord,
  }
};

// modules
mod cl_aligned;

// type Records = Mutex<HashMap<String, ReadRecord>>;


pub fn cl_controller (
  directory: &String,
  cl_aligned_prefix: &String,
  hash_map_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

// pub fn cl_controller (
//   mut hash_map_collection: &mut HashMap<String, ReadRecord>,
//   mut hash_map_anchor: &mut HashMap<String, Vec<String>>,
// ) -> std::io::Result<()> {

  // load reference chromosome aligned reads
  for i in 1..3 {

    let prefix = cl_aligned_prefix.clone();
    let c_directory = directory.clone();

    let c_hash_map_collection = hash_map_collection.clone();
    let c_hash_map_anchor = hash_map_anchor.clone();

    let cl_handle = thread::spawn(move || {

      // let prefix = "SAMN01162223_R".to_string();
      // let prefix = "SAMN02692344_R".to_string();

      // let directory = "/Users/drivas/chapulinTest/".to_string();
      let sufix = ".sorted.sam".to_string();
      let cl_aligned_file = format!("{}{}{}{}", c_directory, prefix, i, sufix);

        cl_aligned::cl_mapper(
          &cl_aligned_file,
          c_hash_map_collection,
          c_hash_map_anchor,
        ).expect(&cl_aligned_file);

    });
    cl_handle.join().unwrap();

      // cl_aligned::cl_mapper(
      //   &cl_aligned_file,
      //   &mut hash_map_collection,
      //   &mut hash_map_anchor,
      // ).expect(&cl_aligned_file);

  }
  Ok(())
}
