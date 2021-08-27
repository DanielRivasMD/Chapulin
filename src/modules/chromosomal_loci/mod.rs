////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::thread;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate features
use crate::Strands;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
pub mod cl_aligned;
pub mod cl_filter;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cl_single_controller(
  directory: String,
  prefix: String,
  hash_map_anchor: alias::RegistryME,
  hash_map_collection: alias::RecordME,
  debug_iteration: i32,
) -> alias::AnyResult {
  // load reference chromosome aligned reads
  for i in 1..=2 {
    let cdirectory = directory.clone();
    let cprefix = prefix.clone();
    let chash_map_collection = hash_map_collection.clone();
    let chash_map_anchor = hash_map_anchor.clone();

    let cl_handle = thread::spawn(move || {
      let sufix = ".sorted.sam".to_string();
      let cl_aligned_file = format!("{}{}{}{}", cdirectory, cprefix, i, sufix);

      cl_aligned::cl_mapper(
        &cl_aligned_file,
        chash_map_anchor,
        chash_map_collection,
        debug_iteration,
      )
      .expect("TODO thread error");
    });
    cl_handle.join().expect("MESSAGE_JOIN");
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cl_paired_controller(
  directory: String,
  prefix: String,
  hash_map_anchor: alias::RegistryME,
  hash_map_collection: alias::RecordME,
  debug_iteration: i32,
) -> alias::AnyResult {
  let cl_aligned_file = format!("{}{}", directory, prefix);

  cl_aligned::cl_mapper(
    &cl_aligned_file,
    hash_map_anchor,
    hash_map_collection,
    debug_iteration,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cl_filter(
  an_registry: alias::RegistryME,
  registry_strand: alias::RegistryStrand,
  hm_me_collection: alias::RecordME,
) -> alias::AnyResult {
  // iterate
  for key in an_registry.lock().unwrap().keys() {
    // TODO: implement parallelism
    // let cl_handle = thread::spawn(|| {
    // declare strand reference
    registry_strand
      .lock()
      .unwrap()
      .insert(key.to_string(), Strands::new());
    // let mut strands = Strands::new();

    // TODO: implement parallel iteration here

    // filter hits
    //////////////////////////////////////////////////
    // select based on likehood of alignment -> MAPQ
    //////////////////////////////////////////////////

    cl_filter::filter(key, &an_registry, &hm_me_collection, &registry_strand);
    // });
    // cl_handle.join().expect("MESSAGE_JOIN");
  }

  // filter based on estimated & false discovery rate threshold
  //////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
