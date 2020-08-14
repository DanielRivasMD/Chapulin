
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod sv_registry;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::structures::{
    sv_chimeric_pair::SVChimericPair,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn sv_controller (
  directory: &str,
  expected_tlen: i32,
  pair_end_reference_alignment: &str,
  hash_map_collection: Arc<Mutex<HashMap<String, SVChimericPair>>>,
  hash_map_anchor: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> anyResult<()> {

  let c_directory = directory.to_string();

  let sv_aligned_file = format!("{}{}", c_directory, pair_end_reference_alignment);
  println!("{}", sv_aligned_file);

  sv_registry::sv_mapper(
    &sv_aligned_file,
    expected_tlen,
    hash_map_collection,
    hash_map_anchor,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
