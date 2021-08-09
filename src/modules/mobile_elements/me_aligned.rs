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
  // AnchorEnum,
  ChrAnchorEnum,
  ExtraValuesEnum,
  MEAnchor,
  MEChimericPair,
  OrientationEnum,
  RawValues,
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
  debug_iteration: i32,
) -> anyResult<()> {
  // load file
  let mut lines = byte_file_reader(&me_bam_file)?;

  // declare initial values
  // local temporary values overwritten each iteration
  // local switches must be declared outside the loop as well as
  // inside to evaluate at last line
  let mut local_switches = LocalSwtiches::new();

  // declare mutable raw values prior to loop
  // so read control can remember
  // it will be overwritten after each iteration
  // but it will remember previous state
  let mut raw_values = RawValues::new();


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



    // SAM line values updated at each iteration
    update!(raw_values, record_line, ChapulinCommonError::Parsing);

    // TODO: load local switches
    // TODO: just clone cigar struct?
    // TODO: to deprecate
    // mobel anchor struct updates EXCEPT orientation & size
    local_switches.mobel_anchor_update(&raw_values);
    // TODO: describe break point signature

    // retrieve mobile element library records
    if let Some(me_record) =
      hm_me_collection.lock().unwrap().get(&raw_values.scaffold)
    {
      raw_values.extra = ExtraValuesEnum::MobelSize(*me_record);
    } else {
      // error!("Mobile element: {:?} is in alignment but not in database", &local_switches.mobel_anchor.mobel);
    }

    // tagging mobel anchor
    // switches get updated by local switches methods
    local_switches.tag();

    // purge read pairs on hashmap (record collection)
    // enter block if
    // read id as changed (through read memory) indicating different batch
    // or previous read is not empty (indicating is not the first line)
    if !(raw_values.read_id.previous == raw_values.read_id.current
      || raw_values.read_id.previous.is_empty())
    {
      // evaluate read batch
      if local_switches.purge_switch {
        hm_record_collection
          .lock()
          .unwrap()
          .remove(&raw_values.read_id.previous);
      }

      // reset purge switch
      // TODO: probably uneccesary? to deprecate
      local_switches.reset_purge();
    }

    // mount data on hashmap (record collection)
    // match on flag (proviral)
    // this check is much faster than using binary interpretor
    match raw_values.flag {
      // primary alignment
      proviral_flag if proviral_flag <= 255 => {
        // insert record if it is not present on hashmap (record collection)
        if !hm_record_collection
          .lock()
          .unwrap()
          .contains_key(&raw_values.read_id.current)
        {
          hm_record_collection
            .lock()
            .unwrap()
            .insert(raw_values.read_id.current.clone(), MEChimericPair::new());

          // if newly inserted tag mobel anchor Read1 & chr anchor Read2
          if let Some(current_record) = hm_record_collection
            .lock()
            .unwrap()
            .get_mut(&raw_values.read_id.current)
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
          .get_mut(&raw_values.read_id.current)
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
          .get_mut(&raw_values.read_id.current)
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
    raw_values.read_id.read_memory();

  }

  // evaluate at end of file
  if local_switches.purge_switch {
    hm_record_collection
      .lock()
      .unwrap()
      .remove(&raw_values.read_id.previous);
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// explicit value assginment to boolean switches
#[derive(Debug, new)]
struct LocalSwtiches {
  #[new(value = "false")]
  mobel_anchor_switch: bool,

  purge_switch: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// extend functionality of raw values locally
trait MEAnchorExt {
  fn mobel_tag(
    &mut self,
    switch: &mut LocalSwtiches,
  ) /* -> String */;
  fn downstream(
    &mut self,
    switch: &mut LocalSwtiches,
  );
  fn upstream(
    &mut self,
    switch: &mut LocalSwtiches,
  );
}

impl MEAnchorExt for RawValues {
  fn mobel_tag(
    &mut self,
    switch: &mut LocalSwtiches,
  ) /* -> String */
  {
    let read_orient = self.read_orientation_get();
    if self.cigar.left_boundry <= ME_LIMIT && read_orient {
      self.upstream(switch);
    // return String::from("upstream");
    } else if self.extra_get() - self.cigar.right_boundry as f64 <=
      ME_LIMIT.into() &&
      !read_orient
    {
      self.downstream(switch);
    // return String::from("downstream");
    } else {
      // TODO: nothing
      // return String::new();
    }
  }

  // change swithces & tag mobile element orientation as downstream
  fn downstream(
    &mut self,
    switch: &mut LocalSwtiches,
  ) {
    switch.switches();
    self.orientation = OrientationEnum::Downstream;
  }

  // change swithces & tag mobile element orientation as upstream
  fn upstream(
    &mut self,
    switch: &mut LocalSwtiches,
  ) {
    switch.switches();
    self.orientation = OrientationEnum::Upstream;
  }
}

// local implementations on local switches
impl LocalSwtiches {
  fn switches(&mut self) {
    self.purge_switch = true;
    self.mobel_anchor_switch = true;
  }

  // TODO: probably uneccesary? to deprecate
  fn reset_purge(&mut self) {
    self.purge_switch = false;
  }

  fn reset_anchor(&mut self) {
    self.mobel_anchor_switch = false;
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: write down tests to assert that data & switches are being updated properly
