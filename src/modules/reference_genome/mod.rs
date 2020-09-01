
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::Result as anyResult;
use std::path::Path;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod ref_cache_read;
mod ref_cache_write;
use super::fasta_read::reference_read;

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn ref_controller (
  directory: &str,
  reference_file: &str,
  hash_map_chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {

  let ref_sequence = format!("{}{}", directory, reference_file);
  let ref_cache = format!("{}.{}.cache", directory, reference_file);

  if Path::new(&ref_cache).exists() {

    // read from cache
    ref_cache_read::read_cache(
      ref_cache,
      hash_map_chr_assembly,
    )?;

  } else {

    // read fasta reference
    let c_chr_assembly = hash_map_chr_assembly.clone();
    reference_read::reference_reader(
      ref_sequence,
      c_chr_assembly,
    )?;

    // write to cache
    ref_cache_write::write_cache(
      ref_cache,
      hash_map_chr_assembly,
    )?;

  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
