
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod me_aligned;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  modules::fasta_read::{
    cache_controller
  },
  utils::structures::{
    me_chimeric_pair::MEChimericPair
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn me_controller (
  directory: &str,
  me_library: &str,
  me_aligned_file: &str,
  hash_map_me_library: Arc<Mutex<HashMap<String, f64>>>,
  hash_map_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
) -> anyResult<()> {


  // load mobile element library
  let ci_hash_map_me_library = Arc::clone(&hash_map_me_library);

  cache_controller::cache_controller(
    directory,
    me_library,
    ci_hash_map_me_library,
  )?;

  // load mobile element aligned reads
  let me_aligned_file = format!("{}{}", directory, me_aligned_file);

  me_aligned::me_identificator(
    &me_aligned_file,
    hash_map_me_library,
    hash_map_collection,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
