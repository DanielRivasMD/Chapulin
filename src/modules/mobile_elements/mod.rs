////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use genomic_structures::MEChimericPair;
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod me_aligned;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn me_controller(
  directory: &str,
  me_aligned_file: &str,
  hash_map_me_library: Arc<Mutex<HashMap<String, f64>>>,
  hash_map_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
) -> anyResult<()> {
  // load mobile element aligned reads
  let me_aligned_file_full = format!("{}{}", directory, me_aligned_file);

  me_aligned::me_identificator(
    &me_aligned_file_full,
    hash_map_me_library,
    hash_map_collection,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
