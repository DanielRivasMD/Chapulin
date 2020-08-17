
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
    sv_chimeric_pair::SVChimericPair,
    chr_anchor::ChrAnchor,
    sv_type::SVType,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  common_error::ChapulinCommonError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn sv_mapper(
  sv_bam_file: &str,
  expected_tlen: i32,
  hm_collection: Arc<Mutex<HashMap<String, SVChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> anyResult<()> {

  // load file
  let mut lines = byte_file_reader(&sv_bam_file)?;

  // declare initial values
  let mut prev_read_id = String::new();
  let mut purge_switch = true;

  // iterate through file
  while let Some(line) = lines.next() {

    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    // update read id
    let read_id = record_line[0].to_string();

    // calculate current values
    let chr = record_line[2].to_string();

    // purge read pairs
    if ! ( prev_read_id == read_id || prev_read_id == "" ) {
      // evaluate read batch
      if purge_switch {
          hm_collection.lock().unwrap().remove(&prev_read_id);
      } else {
        // register chromosome anchors
        if ! an_registry.lock().unwrap().contains_key(&chr) {
          an_registry.lock().unwrap().insert(chr.clone(), Vec::new());
        }
        if let Some(current_chr) = an_registry.lock().unwrap().get_mut(&chr) {
          if ! current_chr.contains(&read_id) {
            current_chr.push(read_id.clone())
          }
        }
      }

      // reset purge switch
      purge_switch = true;
    }

    if ! hm_collection.lock().unwrap().contains_key(&read_id) {
      hm_collection.lock().unwrap().insert((&read_id).to_string(), SVChimericPair::new(SVType::None));

      if let Some(current_record) = hm_collection.lock().unwrap().get_mut(&read_id) {
        load!(current_record, read1, record_line);
      }
    } else if let Some(current_record) = hm_collection.lock().unwrap().get_mut(&read_id) {
      load!(current_record, read2, record_line);
      purge_switch = ! current_record.identificator(expected_tlen);
    }
    prev_read_id = read_id;
  }

  // evaluate at end of file
  if purge_switch {
    hm_collection.lock().unwrap().remove(&prev_read_id);
  }

  println!("File read: {}", &sv_bam_file);
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
