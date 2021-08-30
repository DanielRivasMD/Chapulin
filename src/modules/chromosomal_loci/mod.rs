////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::thread;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::StrandDirection;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
pub mod cl_aligned;
pub mod cl_filter;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cl_single_controller(
  directory: String,
  prefix: String,
  chr_registry: alias::RegistryChr,
  me_record: alias::RecordME,
  debug_iteration: i32,
) -> alias::AnyResult {
  // load reference chromosome aligned reads
  for i in 1..=2 {
    let cdirectory = directory.clone();
    let cprefix = prefix.clone();
    let cme_record = me_record.clone();
    let cchr_registry = chr_registry.clone();

    let cl_handle = thread::spawn(move || {
      let sufix = ".sorted.sam".to_string();
      let cl_aligned_file = format!("{}{}{}{}", cdirectory, cprefix, i, sufix);

      cl_aligned::cl_mapper(
        &cl_aligned_file,
        cchr_registry,
        cme_record,
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
  chr_registry: alias::RegistryChr,
  me_record: alias::RecordME,
  debug_iteration: i32,
) -> alias::AnyResult {
  let cl_aligned_file = format!("{}{}", directory, prefix);

  cl_aligned::cl_mapper(
    &cl_aligned_file,
    chr_registry,
    me_record,
    debug_iteration,
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cl_filter(
  chr_registry: alias::RegistryChr,
  dir_registry: alias::RegistryDir,
  me_record: alias::RecordME,
) -> alias::AnyResult {
  // iterate
  for key in chr_registry.lock().unwrap().keys() {
    // TODO: implement parallelism
    // let cl_handle = thread::spawn(|| {
    // declare strand reference
    dir_registry
      .lock()
      .unwrap()
      .insert(key.to_string(), StrandDirection::new());
    // let mut strands = Strands::new();

    // TODO: implement parallel iteration here

    // filter hits
    //////////////////////////////////////////////////
    // select based on likehood of alignment -> MAPQ
    //////////////////////////////////////////////////

    cl_filter::filter(key, &chr_registry, &dir_registry, &me_record);
    // });
    // cl_handle.join().expect("MESSAGE_JOIN");
  }

  // filter based on estimated & false discovery rate threshold
  //////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
