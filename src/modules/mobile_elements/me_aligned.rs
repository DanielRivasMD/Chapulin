////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use std::collections::HashMap;
use std::str::from_utf8;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  ChrAnchorEnum,
  MEAnchor,
  MEChimericPair,
  CIGAR,
};

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

    // load record line
    load!(read_values, record_line, ChapulinCommonError::Parsing);

    // cigar
    let dc_cigar = CIGAR::loader(&read_values.pv_cigar, read_values.pv_position);

    // TODO: describe break point signature

    // retrieve mobile element library records
    if let Some(me_record) =
      hm_me_collection.lock().unwrap().get(&read_values.mobel)
    {
      read_values.me_size = *me_record;
      // me_size = me_record.me_size;
    }

    // purge read pairs
    if !(read_values.prev_read_id == read_values.read_id
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
    if dc_cigar.left_boundry <= ME_LIMIT && read_values.read_orientation {
      read_values.purge_switch = false;
      read_values.mobel_anchor = true;
      read_values.mobel_orientation = "upstream".to_string();
    } else if read_values.me_size - dc_cigar.right_boundry as f64 <= ME_LIMIT.into()
      && !read_values.read_orientation
    {
      read_values.purge_switch = false;
      read_values.mobel_anchor = true;
      read_values.mobel_orientation = "downstream".to_string();
    }

    // match on proviral flag
    // this check is much faster than using binary interpretor
    match read_values.pv_flag {
      // primary alignment
      pf if pf <= 255 => {
        if !hm_record_collection
          .lock()
          .unwrap()
          .contains_key(&read_values.read_id)
        {
          hm_record_collection.lock().unwrap().insert(
            (&read_values.read_id).to_string(),
            MEChimericPair::new(ChrAnchorEnum::None),
          );

          if let Some(current_record) = hm_record_collection
            .lock()
            .unwrap()
            .get_mut(&read_values.read_id)
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
        } else if let Some(current_record) = hm_record_collection
          .lock()
          .unwrap()
          .get_mut(&read_values.read_id)
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
        if let Some(current_record) = hm_record_collection
          .lock()
          .unwrap()
          .get_mut(&read_values.read_id)
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
    read_values.prev_read_id = read_values.read_id;
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
  #[new(value = "0.")]
  me_size:           f64,
  #[new(default)]
  mobel:             String,
  #[new(value = "false")]
  mobel_anchor:      bool,
  #[new(default)]
  mobel_orientation: String,
  #[new(default)]
  prev_read_id:      String,
  #[new(value = "true")]
  purge_switch:      bool,
  #[new(default)]
  pv_cigar:          String,
  #[new(default)]
  pv_flag:           i32,
  #[new(default)]
  pv_position:       i32,
  #[new(default)]
  read_id:           String,
  #[new(default)]
  read_orientation:  bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
