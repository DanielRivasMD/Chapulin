
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

    // calculate current values
    let read_id = record_line[0].to_string();
    let chr = record_line[2].to_string();

    if hm_collection
      .lock().unwrap()
      .contains_key(&read_id) {

      let mut mapq_switch = false;

      if let Some(current_record) = hm_collection
        .lock().unwrap()
        .get_mut(&read_id) {

        reload!(current_record, read1, record_line);
        reload!(current_record, read2, record_line);

        match current_record.chranch {
          ChrAnchorEnum::Read1 => { mapq_switch = mapq!(current_record, read1); },
          ChrAnchorEnum::Read2 => { mapq_switch = mapq!(current_record, read2); },
          _ => (),
        };

      }

// TODO: consider tagging strand on the fly to avoid postload counting
      if mapq_switch {
        hm_collection
          .lock().unwrap()
          .remove(&read_id);
      } else {
        // register chromosome anchors
        if ! an_registry
          .lock().unwrap()
          .contains_key(&chr) {
          an_registry
            .lock().unwrap()
            .insert(chr.clone(), Vec::new());
        }

        if let Some(current_chr) = an_registry
          .lock().unwrap()
          .get_mut(&chr) {
          if ! current_chr.contains(&read_id.to_string()) {
            current_chr.push(read_id.to_string())
          }
        }
      }

      // TODO: write no found record to error file
    // } else {
    //   ic!(record_line);
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
