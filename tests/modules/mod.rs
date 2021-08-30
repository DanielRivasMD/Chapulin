////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod chromosomal_loci;
mod fasta_read;
mod mobile_elements;
mod peak_identification;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use chapulin::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn insert_me_library(
  mobel_id: String,
  mobel_size: f64,
) -> alias::LibraryME {
  // declare mobile element library
  let amx_me_library = alias::arc_map();

  // insert mobile element library
  amx_me_library.lock().unwrap().insert(mobel_id, mobel_size);

  return amx_me_library;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
