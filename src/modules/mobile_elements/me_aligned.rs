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
  RawValues,
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
  debug_iteration: i32,
) -> anyResult<()> {
  // load file
  let mut lines = byte_file_reader(&me_bam_file)?;

  // declare initial values
  // local temporary values overwritten each iteration
  // local switches must be declared outside loop to evaluate at last line
  let mut local_switches = LocalSwtiches::new();

  // iterate through file
  while let Some(line) = lines.next() {
    // load line into vector
    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    // reset structs
    // overwirte local switches
    local_switches = LocalSwtiches::new();
    // SAM line values declared at each iteration
    // let raw_values = RawValues::load(record_line); //, ChapulinCommonError::Parsing);
    let mut raw_values = RawValues::new();
    update!(raw_values, record_line, ChapulinCommonError::Parsing);

    // collect read id
    reads.read_id = raw_values.read_id.clone();

    // TODO: load local switches
    local_switches.mobel_anchor_update(raw_values.clone());
    // TODO: describe break point signature

    // retrieve mobile element library records
    if let Some(me_record) = hm_me_collection
      .lock()
      .unwrap()
      .get(&local_switches.mobel_anchor.mobel)
    // hm_me_collection.lock().unwrap().get(&raw_values.scaffold)
    {
      local_switches.mobel_anchor.size = *me_record;
    } else {
      error!("Mobile element: {:?} is in alignment but not in database", &local_switches.mobel_anchor.mobel);
    }

    // purge read pairs on hash map (record collection)
    if !(local_switches.prev_read_id == local_switches.read_id
      || local_switches.prev_read_id.is_empty())
    {
      // evaluate read batch
      if local_switches.purge_switch {
        hm_record_collection
          .lock()
          .unwrap()
          .remove(&reads.prev_read_id);
      }

      // reset purge switch
      local_switches.reset_purge();
    }

    // tagging
    // switches get updated by local switches methods
    local_switches.tag();

    // mount data on hash map (record collection)
    // match on flag (proviral)
    // this check is much faster than using binary interpretor
    match raw_values.flag {
      // primary alignment
      proviral_flag if proviral_flag <= 255 => {
        // insert record if it is not present on hash map (record collection)
        if !hm_record_collection
          .lock()
          .unwrap()
          .contains_key(&raw_values.read_id)
        {
          hm_record_collection
            .lock()
            .unwrap()
            .insert((&raw_values.read_id).to_string(), MEChimericPair::new());

          // if newly inserted tag mobel anchor Read1 & chr anchor Read2
          if let Some(current_record) = hm_record_collection
            .lock()
            .unwrap()
            .get_mut(&raw_values.read_id)
          {
            update!(
              current_record,
              read1,
              raw_values,
              local_switches,
              ChapulinCommonError::Parsing
            );
            if local_switches.mobel_anchor_switch {
              current_record.chranch = ChrAnchorEnum::Read2;
            }
          }
        // if already present tag mobel anchor Read2 & chr anchor Read1
        } else if let Some(current_record) = hm_record_collection
          .lock()
          .unwrap()
          .get_mut(&raw_values.read_id)
        {
          update!(
            current_record,
            read2,
            raw_values,
            local_switches,
            ChapulinCommonError::Parsing
          );
          if local_switches.mobel_anchor_switch {
            current_record.chranch = ChrAnchorEnum::Read1;
          }
        }
      }

      // secondary alignment
      proviral_flag if proviral_flag >= 256 => {
        if let Some(current_record) = hm_record_collection
          .lock()
          .unwrap()
          .get_mut(&raw_values.read_id)
        {
          // if sequence field is empty insert ? BUG: is this correct?
          if current_record.read2.sequence.is_empty() {
            update!(
              current_record,
              read1,
              raw_values,
              local_switches,
              ChapulinCommonError::Parsing
            );
            if local_switches.mobel_anchor_switch {
              current_record.chranch = ChrAnchorEnum::Read2;
            }
          } else {
            update!(
              current_record,
              read2,
              raw_values,
              local_switches,
              ChapulinCommonError::Parsing
            );
            if local_switches.mobel_anchor_switch {
              current_record.chranch = ChrAnchorEnum::Read1;
            }
          }
        }
      }

      _ => (),
    }

    // reset anchor switch
    local_switches.reset_anchor();

    // remember previous read
    reads.read_memory();

  }

  // evaluate at end of file
  if local_switches.purge_switch {
    hm_record_collection
      .lock()
      .unwrap()
      .remove(&reads.prev_read_id);
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new)]
struct LocalSwtiches {
  #[new(default)]
  mobel_anchor: MEAnchor,

  #[new(value = "false")]
  mobel_anchor_switch: bool,

  #[new(value = "true")]
  purge_switch: bool,

  #[new(default)]
  read_orientation: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl LocalSwtiches {
  fn mobel_anchor_update(&mut self, raw_values: RawValues) {
    self.mobel_anchor.update(
      raw_values.cigar,
      raw_values.flag,
      raw_values.scaffold,
      // raw_values.orientation,
      raw_values.position,
      // raw_values.size
    );
  }

  fn tag(&mut self) {
    if self.mobel_anchor.cigar.left_boundry <= ME_LIMIT && self.read_orientation {
      self.upstream();
    } else if self.mobel_anchor.size - self.mobel_anchor.cigar.right_boundry as f64 <= ME_LIMIT.into() && !self.read_orientation {
      self.downstream();
    } else {
      // TODO: nothing
    }
  }

  fn upstream(&mut self) {
    self.switches();
    self.mobel_anchor.orientation = "upstream".to_string();
  }

  fn downstream(&mut self) {
    self.switches();
    self.mobel_anchor.orientation = "downstream".to_string();
  }

  fn switches(&mut self) {
    self.purge_switch = false;
    self.mobel_anchor_switch = true;
  }

  fn reset_purge(&mut self) {
    self.purge_switch = true;
  }

  fn reset_anchor(&mut self) {
    self.mobel_anchor_switch = false;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
