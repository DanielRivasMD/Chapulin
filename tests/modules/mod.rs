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

// crate utilities
use chapulin::modules::{
  mobile_elements::me_aligned,
};

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

pub fn load_me_sam(
  me_alignment: &str,
  amx_me_library: alias::LibraryME,
) -> alias::RecordME {
  // declare chimeric mobile element collection
  let amx_me_record = alias::arc_map();

  // declare chimeric mobile element clone
  let camx_me_record = alias::arc_clone(&amx_me_record);

  // identify mobile elements
  me_aligned::me_identificator(me_alignment, amx_me_library, amx_me_record, 0)
    .expect("Error occured at mobile element identificator!");

  return camx_me_record;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
