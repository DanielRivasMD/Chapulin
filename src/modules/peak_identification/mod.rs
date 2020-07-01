
// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::{thread};

// modules
mod pi_mapping;

// crate utilities
use crate::{
  utils::{
    me_chimeric_pair::MEChimericPair
  }
};


pub fn pi_controller(
  hash_map_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
  hash_map_chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
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
    let c_hash_map_chr_assembly = hash_map_chr_assembly.clone();

    let pi_handle = thread::spawn(move || {
      pi_mapping::pi_identifier(
        &okey,
        c_hash_map_collection,
        c_hash_map_anchor,
        c_hash_map_chr_assembly,
      ).expect(&okey);
    });
    pi_handle.join().unwrap();

  }
  // TODO: gather all positions & output a comprenhensive list

  Ok(())
}