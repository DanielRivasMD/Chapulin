////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyResult;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
use crate::modules::fasta_read::{
  fasta_cache_read,
  fasta_cache_write,
  fasta_file_read,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cache_controller(
  subcmd: &str,
  directory: &str,
  fasta_file: &str,
  hash_map_fasta: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {
  let ref_sequence = format!("{}{}", directory, fasta_file);
  let ref_cache = format!("{}.{}.cache", directory, fasta_file);

  if Path::new(&ref_cache).exists() {
    if subcmd == "CR" {
      info!("Cache exists: {}", ref_cache);
    } else {
      // read from cache
      fasta_cache_read::read_cache(&ref_cache, hash_map_fasta)?;
    }
  } else {
    // read fasta reference
    let cfasta = hash_map_fasta.clone();
    fasta_file_read::fasta_reader(&ref_sequence, cfasta)?;

    // write to cache
    fasta_cache_write::write_cache(&ref_cache, hash_map_fasta)?;
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
