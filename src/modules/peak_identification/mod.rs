
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

// crate utilities
use crate::{
  utils::{
    read_record::ReadRecord
  }
};

// modules
mod pi_mapping;


pub fn pi_controller(
  hash_map_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

// pub fn pi_controller(
//   hash_map_collection: &HashMap<String, ReadRecord>,
//   hash_map_anchor: &HashMap<String, Vec<String>>,
// ) -> std::io::Result<()> {

  let mut chromosome_vec = Vec::new();
  for okey in hash_map_anchor.lock().unwrap().keys() {
    let ckey = okey.clone();
    chromosome_vec.push(ckey);
  }

  println!("{}", hash_map_collection.lock().unwrap().len());

  for okey in chromosome_vec {

    let c_hash_map_collection = hash_map_collection.clone();
    let c_hash_map_anchor = hash_map_anchor.clone();

    let pi_handle = thread::spawn(move || {
      pi_mapping::pi_identifier(
        &okey,
        c_hash_map_collection,
        c_hash_map_anchor,
      ).expect(&okey);
    });
    pi_handle.join().unwrap();

  }
  // TODO: gather all positions & output a comprenhensive list

  Ok(())
}