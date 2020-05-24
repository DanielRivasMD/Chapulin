
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// crate utilities
use crate::utils::{
  read_record::ReadRecord
};

// modules
mod me_registry;
mod me_aligned;

pub fn me_controller (
  directory: &String,
  me_library: &String,
  me_aligned_file: &String,
  hash_map_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
) -> std::io::Result<()> {

// pub fn me_controller(
//   mut hash_map_collection: &mut HashMap<String, ReadRecord>,
// ) -> std::io::Result<()> {

  // define files directory
  // let directory = "/Users/drivas/chapulinTest/".to_string();

  // init mobile element library hashmap
  let mut me_collection = HashMap::new();

  // let me_library = "chlSab_ltr_size.txt".to_string();

  // load mobile element library
  let me_library_file = format!("{}{}", directory, me_library);
  me_registry::me_lib_loader(
    &me_library_file,
    &mut me_collection,
  ).expect(&me_library_file);

  // let prefix = "SAMN01162223_".to_string();
  // // let prefix = "SAMN02692344_".to_string();
  // let sufix = "ERV_chlSab_XV.sam".to_string();
  // let me_aligned_file = format!("{}{}{}", directory, prefix, sufix);

  // load mobile element aligned reads
  let me_aligned_file = format!("{}{}", directory, me_aligned_file);

  me_aligned::me_identificator(
    &me_aligned_file,
    hash_map_collection,
    &me_collection,
  ).expect(&me_aligned_file);

  Ok(())
}
