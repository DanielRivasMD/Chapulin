
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
    flag_interpretor::interpretor,
  },
  utils::structures::{
    me_chimeric_pair::MEChimericPair,
    me_library::MElibrary,
    me_anchor::MEAnchor,
    cigar::CIGAR,
    chr_anchor_enum::ChrAnchorEnum,
  },
  settings::{
    constants::ME_LIMIT,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  common_error::ChapulinCommonError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn me_identificator(
  me_bam_file: &str,
  hm_record_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  hm_me_collection: &HashMap<String, MElibrary>,
) -> anyResult<()> {

  // load file
  let mut lines = byte_file_reader(&me_bam_file)?;

  // declare initial values
  let mut prev_read_id = String::new();
  let mut purge_switch = true;
  let mut mobel_anchor = false;
  let mut me_size = 0;
  let mut mobel_orientation = String::new();

  // iterate through file
  while let Some(line) = lines.next() {

    // load line into vector
    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    // update read id
    let read_id = record_line[0].to_string();

    // calculate current values
    let mobel = record_line[2].to_string();
    // let read_seq = record_line[9].to_string();

    // flag & read orientation
    let pv_flag = record_line[1].parse::<i32>().context(ChapulinCommonError::Parsing)?;
    let read_orientation = interpretor(pv_flag, 5);

    // alignment interpretation
    let pv_position = record_line[3].parse::<i32>().context(ChapulinCommonError::Parsing)?;
    let pv_cigar = record_line[5].to_string();
    let dc_cigar = CIGAR::loader(&pv_cigar);
    let adj_left_pos = dc_cigar.left_boundry(pv_position);
    let adj_right_pos = dc_cigar.right_boundry(pv_position);

    // TODO: describe break point signature

    // retrieve mobile element library records
    if let Some(me_record) = hm_me_collection.get(&mobel) {
        me_size = me_record.me_size;
    }

    // purge read pairs
    if ! ( prev_read_id == read_id || prev_read_id == "" ) {
      // evaluate read batch
      if purge_switch {
        hm_record_collection.lock().expect("MESSAGE_MUTEX").remove(&prev_read_id);
      }

      // reset purge switch
      purge_switch = true;
    }

    // tagging
    if adj_left_pos <= ME_LIMIT && read_orientation {
      purge_switch = false;
      mobel_anchor = true;
      mobel_orientation = "upstream".to_string();
    } else if me_size - adj_right_pos <= ME_LIMIT && ! read_orientation {
      purge_switch = false;
      mobel_anchor = true;
      mobel_orientation = "downstream".to_string();
    }

    // match on proviral flag
    // this check is much faster than using binary interpretor
    match pv_flag {

      // primary alignment
      pf if pf <= 255 => {

        if ! hm_record_collection.lock().unwrap().contains_key(&read_id) {
          hm_record_collection.lock().unwrap().insert((&read_id).to_string(), MEChimericPair::new(ChrAnchorEnum::None, ));

          if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
            load!(current_record, read1, record_line, me_size, mobel_orientation);
            if mobel_anchor { current_record.chranch = ChrAnchorEnum::Read2; }
          }
        } else if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
          load!(current_record, read2, record_line, me_size, mobel_orientation);
          if mobel_anchor { current_record.chranch = ChrAnchorEnum::Read1; }
        }
      },

      // secondary alignment
      pf if pf >= 256 => {

        if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
          if current_record.read2.sequence == "" {
            load!(current_record, read1, record_line, me_size, mobel_orientation);
            if mobel_anchor { current_record.chranch = ChrAnchorEnum::Read2; }
          } else {
            load!(current_record, read2, record_line, me_size, mobel_orientation);
            if mobel_anchor { current_record.chranch = ChrAnchorEnum::Read1; }
          }
        }
      },

      _ => (),
    }

    // reset anchor switch
    mobel_anchor = false;
    prev_read_id = read_id;
  }

  // evaluate at end of file
  if purge_switch {
    hm_record_collection.lock().unwrap().remove(&prev_read_id);
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
