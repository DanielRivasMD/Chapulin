////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::ActivateExt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
pub mod me_aligned;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn me_controller(
  directory: &str,
  me_aligned_file: &str,
  hash_map_me_library: alias::LibraryME,
  hash_map_collection: alias::RecordME,
  debug_iteration: i32,
) -> alias::AnyResult {
  // load mobile element aligned reads
  let me_aligned_file_full = format!("{}{}", directory, me_aligned_file);

  me_aligned::me_identificator(
    &me_aligned_file_full,
    hash_map_me_library,
    hash_map_collection,
    debug_iteration,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// majority of reads will be purged
// explicit value assginment to boolean switches
#[derive(Debug, new)]
struct LocalSwtiches {
  // control whether read batches will be removed
  // majority of records will be removed
  // keep active unless encounter mobile element compatible features
  // re activate only after read batch evaluation
  #[new(value = "true")]
  purge: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// local implementations on local switches
impl LocalSwtiches {
  fn switches(&mut self) {
    self.purge.deactivate();
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
