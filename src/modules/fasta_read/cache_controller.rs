
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::Result as anyResult;
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
use crate::modules::fasta_read::fasta_file_read;
use crate::modules::fasta_read::fasta_cache_read;
use crate::modules::fasta_read::fasta_cache_write;

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn cache_controller (
  directory: &str,
  fasta_file: &str,
  hash_map_fasta: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {

  let ref_sequence = format!("{}{}", directory, fasta_file);
  let ref_cache = format!("{}.{}.cache", directory, fasta_file);

  if Path::new(&ref_cache).exists() {

    // read from cache
    fasta_cache_read::read_cache(
      &ref_cache,
      hash_map_fasta,
    )?;

  } else {

    // read fasta reference
    let c_fasta = hash_map_fasta.clone();
    fasta_file_read::fasta_reader(
      &ref_sequence,
      c_fasta,
    )?;

    // write to cache
    fasta_cache_write::write_cache(
      &ref_cache,
      hash_map_fasta,
    )?;

  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
