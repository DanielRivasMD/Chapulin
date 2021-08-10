////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::str::from_utf8;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  ChrAnchor,
  ChrAnchorEnum,
  MEChimericPair,
  RawValues,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  settings::constants::MAPQ,
  utils::io::file_reader::byte_file_reader,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Map chromosomal loci.
pub fn cl_mapper(
  cl_bam_file: &str,
  errata: &str,
  hm_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> anyResult<()> {
  let fl_write = format!("{}.err", errata);
  let _fl =
    File::create(&fl_write).context(ChapulinCommonError::CreateFile {
      f: fl_write,
    })?;

  // load file
  let mut lines = byte_file_reader(&cl_bam_file)?;

  // iterate through file
  while let Some(line) = lines.next() {
    // load line into vector
    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    // reset mapq switch
    let mut mapq_switch = false;

    // SAM line values declared at each iteration
    let raw_values = RawValues::load(record_line)?;

    // TODO: read supplementary fields for additional information & load on
    // struct

    // if read id is present on hashmap (record collection)
    if hm_record_collection
      .lock()
      .unwrap()
      .contains_key(&raw_values.read_id.current)
    {
      // reset switch
      mapq_switch.deactivate();

      if let Some(current_record) = hm_record_collection
        .lock()
        .unwrap()
        .get_mut(&raw_values.read_id.current)
      {
        // load chromosomal anchoring data
        load!(current_record, raw_values, read1);
        load!(current_record, raw_values, read2);

        // evaluate mapq
        match current_record.chranch {
          ChrAnchorEnum::Read1 => {
            mapq_switch = mapq!(current_record, read1);
          }
          ChrAnchorEnum::Read2 => {
            mapq_switch = mapq!(current_record, read2);
          }
          _ => (),
        };
      }

      // IDEA: consider tagging strand on the fly to avoid postload counting
        hm_collection.lock().unwrap().remove(&raw_values.read_id);
      if mapq_switch {
      } else {
        // register chromosome anchors
        if !an_registry
          .lock()
          .unwrap()
          .contains_key(&raw_values.scaffold)
        {
          an_registry
            .lock()
            .unwrap()
            .insert(raw_values.scaffold.clone(), Vec::new());
        }

        if let Some(current_chr) =
          an_registry.lock().unwrap().get_mut(&raw_values.scaffold)
        {
          if !current_chr.contains(&raw_values.read_id.current.to_string()) {
            current_chr.push(raw_values.read_id.current.to_string())
          }
        }
      }
    } else {
      // TODO: all records are going here. investigate the reason
      // ic!(record_line);
      // fl.write_all(record_line[0].to_string().as_bytes()).
      // context(ChapulinCommonError::WriteFile{ f: record_line[0].
      // to_string() })?;
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

trait ActivateExt {
  fn activate(&mut self);
  fn deactivate(&mut self);
// mount current data on hashmap (record collection)
fn mount(
  raw_values: RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
  mut file_out: &File,
) -> anyResult<()> {
  // if read id is present on hashmap (record collection)
  if hm_record_collection
    .lock()
    .unwrap()
    .contains_key(&raw_values.read_id.current)
  {
    // load chromosomal anchoring data
    // check whether sequence or reverse sequence is equal
    // BUG: palindromic reads?
    load(hm_record_collection, &raw_values);

    // register
    register(raw_values, hm_record_collection, an_registry);
  } else {
    // TODO: all records are going here. investigate the reason
    file_out
      .write_all(raw_values.read_id.current.as_bytes())
      .context(ChapulinCommonError::WriteFile {
        f: raw_values.read_id.current,
      })?;
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// load chromosomal anchor data on mobile element chimeric pair
fn load(
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  raw_values: &RawValues,
) {
  if let Some(current_record) = hm_record_collection
    .lock()
    .unwrap()
    .get_mut(&raw_values.read_id.current)
  {
    load!(current_record, *raw_values, read1);
    load!(current_record, *raw_values, read2);
  }
}

impl ActivateExt for bool {
  fn activate(&mut self) {
    *self = true;
///////////////////////////////////////////////////////////////////////////////////////////////////

  }

  fn deactivate(&mut self) {
    *self = false;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
