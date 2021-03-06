////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};
use std::thread;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  MEChimericPair,
  SVChimericPair,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod pi_me_mapping;
mod pi_sv_mapping;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pi_me_controller(
  output: String,
  errata: String,
  hash_map_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
  hash_map_chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {
  let chromosome_vec =
    chr_constructor(hash_map_anchor.clone(), hash_map_chr_assembly.clone());

  for okey in chromosome_vec {
    let coutput = output.clone();
    let cerrata = errata.clone();
    let chash_map_collection = hash_map_collection.clone();
    let chash_map_anchor = hash_map_anchor.clone();
    let chash_map_chr_assembly = hash_map_chr_assembly.clone();

    let pi_me_handle = thread::spawn(move || {
      pi_me_mapping::pi_me_identifier(
        &okey,
        &coutput,
        &cerrata,
        chash_map_collection,
        chash_map_anchor,
        chash_map_chr_assembly,
      )
      .expect("TODO thread error");
    });
    pi_me_handle.join().expect("MESSAGE_JOIN");
  }
  // TODO: gather all positions & output a comprenhensive list

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pi_sv_controller(
  _output: String,
  _errata: String,
  hash_map_collection: Arc<Mutex<HashMap<String, SVChimericPair>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
  hash_map_chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {
  let chromosome_vec =
    chr_constructor(hash_map_anchor.clone(), hash_map_chr_assembly.clone());

  for okey in chromosome_vec {
    let chash_map_collection = hash_map_collection.clone();
    let chash_map_anchor = hash_map_anchor.clone();
    let chash_map_chr_assembly = hash_map_chr_assembly.clone();

    let pi_sv_handle = thread::spawn(move || {
      pi_sv_mapping::pi_sv_identifier(
        &okey,
        chash_map_collection,
        chash_map_anchor,
        chash_map_chr_assembly,
      )
      .expect("TODO thread error");
    });
    pi_sv_handle.join().expect("MESSAGE_JOIN");
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn chr_constructor(
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
  hash_map_chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> Vec<String> {
  // iterate on reference chromosomes
  let mut chromosome_vec = Vec::new();
  for okey in hash_map_chr_assembly.lock().unwrap().keys() {
    let ckey = okey.clone();

    if hash_map_anchor.lock().unwrap().contains_key(okey) {
      chromosome_vec.push(ckey);
    }
  }

  chromosome_vec
}

////////////////////////////////////////////////////////////////////////////////////////////////////
