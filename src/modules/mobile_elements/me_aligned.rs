////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use genomic_structures::{
  interpretor,
  ChrAnchorEnum,
  MEAnchor,
  MEChimericPair,
};
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::CIGAR;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  settings::constants::ME_LIMIT,
  utils::io::file_reader::byte_file_reader,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: extract features from fasta other than sequence length
pub fn me_identificator(
  me_bam_file: &str,
  hm_me_collection: Arc<Mutex<HashMap<String, f64>>>,
  // hm_me_collection: Arc<Mutex<HashMap<String, MElibrary>>>,
  hm_record_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
) -> anyResult<()> {
  // load file
  let mut lines = byte_file_reader(&me_bam_file)?;

  // declare initial values
  let mut read_values = ReadValues::new();

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
    let pv_flag = record_line[1]
      .parse::<i32>()
      .context(ChapulinCommonError::Parsing)?;
    let read_orientation = interpretor(pv_flag, 5);

    // alignment interpretation
    let pv_position = record_line[3]
      .parse::<i32>()
      .context(ChapulinCommonError::Parsing)?;
    let pv_cigar = record_line[5].to_string();
    let dc_cigar = CIGAR::loader(&pv_cigar);
    let (adj_left_pos, adj_right_pos) = dc_cigar.boundries(pv_position);

    // TODO: describe break point signature

    // retrieve mobile element library records
    if let Some(me_record) = hm_me_collection.lock().unwrap().get(&mobel) {
      read_values.me_size = *me_record;
      // me_size = me_record.me_size;
    }

    // purge read pairs
    if !(read_values.prev_read_id == read_id
      || read_values.prev_read_id.is_empty())
    {
      // evaluate read batch
      if read_values.purge_switch {
        hm_record_collection
          .lock()
          .unwrap()
          .remove(&read_values.prev_read_id);
      }

      // reset purge switch
      read_values.purge_switch = true;
    }

    // tagging
    if adj_left_pos <= ME_LIMIT && read_orientation {
      read_values.purge_switch = false;
      read_values.mobel_anchor = true;
      read_values.mobel_orientation = "upstream".to_string();
    } else if read_values.me_size - adj_right_pos as f64 <= ME_LIMIT.into()
      && !read_orientation
    {
      read_values.purge_switch = false;
      read_values.mobel_anchor = true;
      read_values.mobel_orientation = "downstream".to_string();
    }

    // match on proviral flag
    // this check is much faster than using binary interpretor
    match pv_flag {
      // primary alignment
      pf if pf <= 255 => {
        if !hm_record_collection.lock().unwrap().contains_key(&read_id) {
          hm_record_collection.lock().unwrap().insert(
            (&read_id).to_string(),
            MEChimericPair::new(ChrAnchorEnum::None),
          );

          if let Some(current_record) =
            hm_record_collection.lock().unwrap().get_mut(&read_id)
          {
            load!(
              current_record,
              read1,
              record_line,
              read_values.me_size,
              read_values.mobel_orientation,
              ChapulinCommonError::Parsing
            );
            if read_values.mobel_anchor {
              current_record.chranch = ChrAnchorEnum::Read2;
            }
          }
        } else if let Some(current_record) =
          hm_record_collection.lock().unwrap().get_mut(&read_id)
        {
          load!(
            current_record,
            read2,
            record_line,
            read_values.me_size,
            read_values.mobel_orientation,
            ChapulinCommonError::Parsing
          );
          if read_values.mobel_anchor {
            current_record.chranch = ChrAnchorEnum::Read1;
          }
        }
      }

      // secondary alignment
      pf if pf >= 256 => {
        if let Some(current_record) =
          hm_record_collection.lock().unwrap().get_mut(&read_id)
        {
          if current_record.read2.sequence.is_empty() {
            load!(
              current_record,
              read1,
              record_line,
              read_values.me_size,
              read_values.mobel_orientation,
              ChapulinCommonError::Parsing
            );
            if read_values.mobel_anchor {
              current_record.chranch = ChrAnchorEnum::Read2;
            }
          } else {
            load!(
              current_record,
              read2,
              record_line,
              read_values.me_size,
              read_values.mobel_orientation,
              ChapulinCommonError::Parsing
            );
            if read_values.mobel_anchor {
              current_record.chranch = ChrAnchorEnum::Read1;
            }
          }
        }
      }

      _ => (),
    }

    // reset anchor switch
    read_values.mobel_anchor = false;
    read_values.prev_read_id = read_id;
  }

  // evaluate at end of file
  if read_values.purge_switch {
    hm_record_collection
      .lock()
      .unwrap()
      .remove(&read_values.prev_read_id);
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new)]
struct ReadValues {
  #[new(default)]
  prev_read_id:      String,
  #[new(value = "true")]
  purge_switch:      bool,
  #[new(value = "false")]
  mobel_anchor:      bool,
  #[new(value = "0.")]
  me_size:           f64,
  #[new(default)]
  mobel_orientation: String,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
