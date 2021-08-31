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
    cl_filter,
  },
  mobile_elements::me_aligned,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn insert_me_library(
  mobel_id: String,
  mobel_size: f64,
) -> alias::LibraryME {
  // arc
  //////////////////////////////////////////////////

  // mobile element library
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
  // arc
  //////////////////////////////////////////////////

  // record
  let amx_me_record = alias::arc_map();

  // clones
  //////////////////////////////////////////////////

  // record clone
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
  // arc
  //////////////////////////////////////////////////

  // anchor registry
  let amx_chr_registry = alias::arc_map();

  // clones
  //////////////////////////////////////////////////

  // anchor registry clone
  let camx_chr_registry = alias::arc_clone(&amx_chr_registry);

  // record clone
  let camx_me_record_cl = alias::arc_clone(&amx_me_record);

  // record assertion clone
  let camx_me_record_as = alias::arc_clone(&amx_me_record);

  // map chromosomal loci
  cl_aligned::cl_mapper(cl_alignment, amx_chr_registry, camx_me_record_cl, 0)
    .expect("Error occured at chromosomal loci mapper!");

  return (camx_me_record_as, camx_chr_registry);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn filter_cl(
  scaffold: &str,
  amx_chr_registry: alias::RegistryChr,
  amx_me_record: alias::RecordME,
) -> (alias::RecordME, alias::RegistryDir) {
  // arc
  //////////////////////////////////////////////////

  // direction registry clone
  let amx_dir_registry = alias::arc_map();

  // clones
  //////////////////////////////////////////////////

  // registry clone
  let camx_chr_registry = alias::arc_clone(&amx_chr_registry);

  // direction assertion clone
  let camx_dir_registry_as = alias::arc_clone(&amx_dir_registry);

  // record clone
  let camx_me_record_cl = alias::arc_clone(&amx_me_record);

  // record assertion clone
  let camx_me_record_as = alias::arc_clone(&amx_me_record);

  // filter chromosomal loci
  cl_filter::filter(
    scaffold,
    &camx_chr_registry,
    &amx_dir_registry,
    &camx_me_record_cl,
  );

  return (camx_me_record_as, camx_dir_registry_as);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
