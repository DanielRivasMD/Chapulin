////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::thread;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
pub mod cl_aligned;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cl_single_controller(
  directory: String,
  prefix: String,
  errata: String,
  hash_map_collection: alias::RecordME,
  hash_map_anchor: alias::RegistryME,
  debug_iteration: i32,
) -> alias::AnyResult {
  // load reference chromosome aligned reads
  for i in 1..=2 {
    let cdirectory = directory.clone();
    let cprefix = prefix.clone();
    let cerrata = errata.clone();
    let chash_map_collection = hash_map_collection.clone();
    let chash_map_anchor = hash_map_anchor.clone();

    let cl_handle = thread::spawn(move || {
      let sufix = ".sorted.sam".to_string();
      let cl_aligned_file = format!("{}{}{}{}", cdirectory, cprefix, i, sufix);

      cl_aligned::cl_mapper(
        &cl_aligned_file,
        &cerrata,
        chash_map_collection,
        chash_map_anchor,
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
  errata: String,
  hash_map_collection: alias::RecordME,
  hash_map_anchor: alias::RegistryME,
  debug_iteration: i32,
) -> alias::AnyResult {
  let cl_aligned_file = format!("{}{}", directory, prefix);

  cl_aligned::cl_mapper(
    &cl_aligned_file,
    &errata,
    hash_map_collection,
    hash_map_anchor,
    debug_iteration,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
