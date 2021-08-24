////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: since modules are functional, consider implementing error handlers per
// function. this assumes that error handlers can be efficiently tested

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

    // debugger counter
    ct += 1;

    // SAM line values declared at each iteration
    let raw_values = RawValues::load(record_line)?;

    // TODO: read supplementary fields for additional information & load on

    // load & register records
    raw_values.mount(&hm_record_collection, &an_registry)?;

    if ct > debug_iteration && debug_iteration > 0 {
      break;
    }
  }

  println!("{:#?}", &hm_record_collection.lock().unwrap().keys());
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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

impl MountExt for RawValues {
  // mount current data on hashmap (record collection)
  fn mount(
    self,
    hm_record_collection: &alias::RecordME,
    an_registry: &alias::RegistryME,
  ) -> alias::AnyResult {
    // if read id is present on hashmap (record collection)
    if hm_record_collection
      .lock()
      .unwrap()
      .contains_key(&self.read_id.current)
    {
      // load chromosomal anchoring data
      // check whether sequence or reverse sequence is equal
      // BUG: palindromic reads?
      self.load(hm_record_collection);

      // register
      self.register(an_registry);
      // } else {
      //   // TODO: all records are going here. investigate the reason
      //   file_out
      //     .write_all(self.read_id.current.as_bytes())
      //     .context(ChapulinCommonError::WriteFile {
      //       f: self.read_id.current,
      //     })?;
    }

    Ok(())
  }

  // TODO: why are the reads not in order. also, this function should account
  // for that fact since it must support single-end alignments as well
  // IDEA: consider tagging strand on the fly to avoid postload counting
  // BUG: this switch must contain memory, otherwise it'll delete all read2
  // load chromosomal anchor data on mobile element chimeric pair
  fn load(
    &self,
    hm_record_collection: &alias::RecordME,
  ) {
    if let Some(current_record) = hm_record_collection
      .lock()
      .unwrap()
      .get_mut(&self.read_id.current)
    {
      load!( chromosomal |> current_record; *self; read1 );
      load!( chromosomal |> current_record; *self; read2 );
    }
  }

  // register read id on scaffold
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

    // count anchor
    // }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
