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
    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    // calculate current values
    let read_id = record_line[0].to_string();
    let chr = record_line[2].to_string();

    // TODO: read supplementary fields for additional information & load on
    // struct

    if hm_collection.lock().unwrap().contains_key(&read_id) {
      let mut mapq_switch = false;

      if let Some(current_record) =
        hm_collection.lock().unwrap().get_mut(&read_id)
      {
        // load chromosomal anchoring data
        reload!(current_record, read1, record_line);
        reload!(current_record, read2, record_line);

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
      if mapq_switch {
        hm_collection.lock().unwrap().remove(&read_id);
      } else {
        // register chromosome anchors
        if !an_registry.lock().unwrap().contains_key(&chr) {
          an_registry.lock().unwrap().insert(chr.clone(), Vec::new());
        }

        if let Some(current_chr) = an_registry.lock().unwrap().get_mut(&chr) {
          if !current_chr.contains(&read_id.to_string()) {
            current_chr.push(read_id.to_string())
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
