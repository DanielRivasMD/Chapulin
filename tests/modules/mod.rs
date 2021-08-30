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
  chromosomal_loci::{
    cl_aligned,
  },
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

pub fn load_cl_sam(
  cl_alignment: &str,
  amx_me_record: alias::RecordME,
) -> (alias::RecordME, alias::RegistryChr) {
  // declare anchor registry
  let amx_chr_registry = alias::arc_map();

  // declare anchor registry aligned clone
  let camx_chr_registry = alias::arc_clone(&amx_chr_registry);

  // declare chimeric chromosomal loci clone
  let camx_me_record_cl = alias::arc_clone(&amx_me_record);

  // declare assertion clone
  let camx_me_record_as = alias::arc_clone(&amx_me_record);

  // map chromosomal loci
  cl_aligned::cl_mapper(cl_alignment, amx_chr_registry, camx_me_record_cl, 0)
    .expect("Error occured at chromosomal loci mapper!");

  return (camx_me_record_as, camx_chr_registry);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
