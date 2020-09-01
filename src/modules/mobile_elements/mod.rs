
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::Result as anyResult;
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
use super::fasta_read::fasta_file_read;
use super::fasta_read::fasta_cache_read;
use super::fasta_read::fasta_cache_write;

mod me_aligned;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
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
  let me_library_file = format!("{}{}", directory, me_library);
  let me_library_cache = format!("{}{}.cache", directory, me_library);

  if Path::new(&me_library_cache).exists() {

    // read from cache
    let cr_me_library = hash_map_me_library.clone();
    fasta_cache_read::read_cache(
      &me_library_file,
      cr_me_library,
    )?;

  } else {

    // read fasta reference
    let cr_me_library = hash_map_me_library.clone();
    fasta_file_read::fasta_reader(
      &me_library_file,
      cr_me_library,
    )?;

    // write to cache
    let cw_me_library = hash_map_me_library.clone();
    fasta_cache_write::write_cache(
      &me_library_file,
      cw_me_library,
    )?;

  }

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
