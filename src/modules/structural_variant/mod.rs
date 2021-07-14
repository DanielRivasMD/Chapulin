////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use genomic_structures::SVChimericPair;
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod sv_registry;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn sv_controller(
  directory: &str,
  expected_tlen: i32,
  pair_end_reference_alignment: &str,
  hash_map_collection: Arc<Mutex<HashMap<String, SVChimericPair>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> anyResult<()> {
  let sv_aligned_file = format!("{}{}", directory, pair_end_reference_alignment);

  sv_registry::sv_mapper(
    &sv_aligned_file,
    expected_tlen,
    hash_map_collection,
    hash_map_anchor,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
