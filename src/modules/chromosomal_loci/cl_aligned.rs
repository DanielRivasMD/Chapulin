
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::str::{from_utf8};
use anyhow::{Context};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::functions::{
    file_reader::byte_file_reader,
  },
  utils::structures::{
    me_chimeric_pair::MEChimericPair,
    chr_anchor::ChrAnchor,
    chr_anchor_enum::ChrAnchorEnum,
  },
  settings::{
    constants::MAPQ,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  common_error::ChapulinCommonError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn cl_mapper(
  cl_bam_file: &str,
  hm_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> anyResult<()> {

  // load file
  let mut lines = byte_file_reader(&cl_bam_file)?;

  // iterate through file
  while let Some(line) = lines.next() {

    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    if hm_collection.lock().unwrap().contains_key(record_line[0]) {

      let mut mapq_switch = false;

      if let Some(current_record) = hm_collection.lock().unwrap().get_mut(record_line[0]) {

        cl_load!(current_record, read1, record_line);
        cl_load!(current_record, read2, record_line);

        match current_record.chranch {
          ChrAnchorEnum::Read1 => { mapq_switch = cl_mapq!(current_record, read1); },
          ChrAnchorEnum::Read2 => { mapq_switch = cl_mapq!(current_record, read2); },
          _ => (),
        };

      }

      if mapq_switch {
        hm_collection.lock().unwrap().remove(record_line[0]);
      } else {
        // register chromosome anchors
        if ! an_registry.lock().unwrap().contains_key(record_line[2]) {
          an_registry.lock().unwrap().insert(record_line[2].to_string(), Vec::new());
        }

        if let Some(current_chr) = an_registry.lock().unwrap().get_mut(record_line[2]) {
          if ! current_chr.contains(&record_line[0].to_string()) {
            current_chr.push(record_line[0].to_string())
          }
        }
      }
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
