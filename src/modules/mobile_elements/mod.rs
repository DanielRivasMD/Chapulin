
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod me_registry;
mod me_aligned;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::{
    me_chimeric_pair::MEChimericPair
  }
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn me_controller (
  directory: &String,
  me_library: &String,
  me_aligned_file: &String,
  hash_map_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
) -> std::io::Result<()> {

  // init mobile element library hashmap
  let mut me_collection = HashMap::new();

  // load mobile element library
  let me_library_file = format!("{}{}", directory, me_library);
  me_registry::me_lib_loader(
    &me_library_file,
    &mut me_collection,
  )?;

  // load mobile element aligned reads
  let me_aligned_file = format!("{}{}", directory, me_aligned_file);

  me_aligned::me_identificator(
    &me_aligned_file,
    hash_map_collection,
    &me_collection,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
