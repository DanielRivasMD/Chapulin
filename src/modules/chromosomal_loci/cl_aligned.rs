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

    // reset structs
    let mut local_switches = LocalSwtiches::new();
    // SAM line values declared at each iteration
    let raw_values = RawValues::load(record_line); //, ChapulinCommonError::Parsing);
    // let mut raw_values = load!(record_line, ChapulinCommonError::Parsing);

    // TODO: read supplementary fields for additional information & load on
    // struct

    if hm_collection
      .lock()
      .unwrap()
      .contains_key(&raw_values.read_id)
    {
      local_switches.mapq_switch = false;

      if let Some(current_record) =
        hm_collection.lock().unwrap().get_mut(&raw_values.read_id)
      {
        // load chromosomal anchoring data
        reload!(current_record, read1, raw_values);
        reload!(current_record, read2, raw_values);

        // evaluate mapq
        match current_record.chranch {
          ChrAnchorEnum::Read1 => {
            local_switches.mapq_switch = mapq!(current_record, read1);
          }
          ChrAnchorEnum::Read2 => {
            local_switches.mapq_switch = mapq!(current_record, read2);
          }
          _ => (),
        };
      }

      // IDEA: consider tagging strand on the fly to avoid postload counting
      if local_switches.mapq_switch {
        hm_collection.lock().unwrap().remove(&raw_values.read_id);
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
          if !current_chr.contains(&raw_values.read_id.to_string()) {
            current_chr.push(raw_values.read_id.to_string())
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

#[derive(Debug, new)]
struct LocalSwtiches {
  #[new(default)]
  mapq_switch: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
