////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use std::str::from_utf8;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  ChrAnchor,
  RawValues,
  Sequence,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::io::file_reader::byte_file_reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: read supplementary fields for additional information & load on
// only load & register chromosomal loci. filtering is done downstream
// the reason is to widen compatibility with single read alignment files
/// Map chromosomal loci.
pub fn cl_mapper(
  cl_bam_file: &str,
  an_registry: alias::RegistryME,
  hm_record_collection: alias::RecordME,
  debug_iteration: i32,
) -> alias::AnyResult {
  // load file
  let mut lines = byte_file_reader(&cl_bam_file)?;

  // counter for debugger parameter
  let mut ct = 0;

  // iterate through file
  while let Some(line) = lines.next() {
    // load line into vector
    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    // omit incomplete records
    if record_line.len() < 11 {
      continue;
    }

    // debugger counter
    ct += 1;

    // SAM line values loaded at each iteration
    // this implies no memory is held about other records &
    // data is selected on current values
    let raw_values = RawValues::load(record_line)?;

    // load & register records
    // select pair on read id & select read on sequence | reverse complement
    // register on hashmap with scaffold ids as keys for parallelization
    // do not count nor tag on the fly since no filtering is done here
    raw_values.mount(&hm_record_collection, &an_registry)?;

    if ct > debug_iteration && debug_iteration > 0 {
      break;
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// extend functionality of raw values locally
trait MountExt {
  fn mount(
    self,
    hm_record_collection: &alias::RecordME,
    an_registry: &alias::RegistryME,
  ) -> alias::AnyResult;

  fn load(
    &self,
    hm_record_collection: &alias::RecordME,
  );

  fn register(
    self,
    an_registry: &alias::RegistryME,
  );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// mount, load & register
impl MountExt for RawValues {
  // mount current data on hashmap (record collection)
  fn mount(
    self,
    hm_record_collection: &alias::RecordME,
    an_registry: &alias::RegistryME,
  ) -> alias::AnyResult {
    // if read pair id is present on hashmap (record collection)
    if hm_record_collection
      .lock()
      .unwrap()
      .contains_key(&self.read_id.current)
    {
      // load chromosomal anchoring data
      // check sequence or reverse complement to select read
      // BUG: palindromic reads?
      self.load(hm_record_collection);

      // register read pair id on hashmap for parallelization
      self.register(an_registry);
    }

    Ok(())
  }

  // load records on hashmap (record collection)
  fn load(
    &self,
    hm_record_collection: &alias::RecordME,
  ) {
    if let Some(current_record) = hm_record_collection
      .lock()
      .unwrap()
      .get_mut(&self.read_id.current)
    {
      // select based on sequence | reverse complement
      load!( chromosomal |> current_record; *self; read1 );
      load!( chromosomal |> current_record; *self; read2 );
    }
  }

  // register read pair id on scaffold
  fn register(
    self,
    an_registry: &alias::RegistryME,
  ) {
    // register chromosome anchors
    if !an_registry.lock().unwrap().contains_key(&self.scaffold) {
      // clone scaffold value here
      an_registry
        .lock()
        .unwrap()
        .insert(self.scaffold.clone(), Vec::new());
    }

    if let Some(current_chr) =
      an_registry.lock().unwrap().get_mut(&self.scaffold)
    {
      // verify whether vector contains entry
      if !current_chr.contains(&self.read_id.current) {
        // observe that value of the current read is moved here
        current_chr.push(self.read_id.current)
      }
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
