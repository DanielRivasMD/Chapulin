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
  ExtraValuesEnum,
  MEAnchor,
  MEChimericPair,
  OrientationEnum,
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
  // local temporary values are controlled by implementations
  // local switches must be declared outside the loop
  // to keep memory of previous iterations as well as
  // to evaluate at last line
  let mut local_switches = LocalSwtiches::new();

  // declare mutable raw values prior to loop
  // so read control can remember
  // it will be overwritten after each iteration
  // but it will retain previous state
  let mut raw_values = RawValues::new();

  // counter for debugger parameter
  let mut ct = 0;

  // iterate through file
  while let Some(line) = lines.next() {
    // load line into vector
    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    // debugger counter
    ct += 1;

    // SAM line values updated at each iteration
    // observe that raw values holds read control
    // for keeping the state of read batch
    raw_values.update(record_line)?;

    // TODO: describe break point signature

    // retrieve mobile element library records
    me_get(&mut raw_values, &hm_me_collection);

    // tagging mobel anchor
    // switches get updated by local switches methods
    raw_values.mobel_tag(&mut local_switches);

    // purge read pairs on hashmap (record collection)
    batch_purge(&mut local_switches, &mut raw_values, &hm_record_collection);

    // mount current data on hashmap (record collection)
    mount(&local_switches, &mut raw_values, &hm_record_collection)?;

    // reset orientation
    raw_values.reset_orientation();

    // reset anchor switch
    local_switches.deactivate_anchor();

    // remember previous read
    raw_values.read_id.read_memory();

    if ct > debug_iteration && debug_iteration > 0 {
      break;
    }
  }

  // evaluate at end of file
  purge(&local_switches, &raw_values, &hm_record_collection);

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// majority of reads will be purged
// explicit value assginment to boolean switches
#[derive(Debug, new)]
struct LocalSwtiches {
  // keep track whether read in pair is compatible for mobile element anchoring
  // activate when encounter mobile element compatible features
  // reset at end of each iteration
  #[new(value = "false")]
  mobel_anchor_switch: bool,

  // control whether read batches will be removed
  // majority of records will be removed
  // keep active unless encounter mobile element compatible features
  // re activate only after read batch evaluation
  #[new(value = "true")]
  purge_switch: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// local implementations on local switches
impl LocalSwtiches {
  fn switches(&mut self) {
    self.activate_anchor();
    self.deactivate_purge();
  }

  fn activate_purge(&mut self) {
    self.purge_switch = true;
  }

  fn deactivate_purge(&mut self) {
    self.purge_switch = false;
  }

  fn activate_anchor(&mut self) {
    self.mobel_anchor_switch = true;
  }

  fn deactivate_anchor(&mut self) {
    self.mobel_anchor_switch = false;
  }
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

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MEAnchorExt for RawValues {
  // since read orientation can be calculated with only
  // on current values on the fly through function
  fn mobel_tag(
    &mut self,
    switch: &mut LocalSwtiches,
  ) /* -> String */
  {
    // assign true when read is aligned on
    // reversed strand in relation to assembly
    // otherwise false
    let read_orient = self.read_orientation_get();
    if self.cigar.left_boundry <= ME_LIMIT && read_orient {
      // println!("UPSTREAM: {} <= {}", self.cigar.left_boundry, ME_LIMIT);
      self.upstream(switch);
    // return String::from("upstream");
    } else if self.extra_get() - self.cigar.right_boundry as f64 <=
      ME_LIMIT.into() &&
      !read_orient
    {
      // println!(
      //   "DOWNSTREAM: {} - {} = {} <= {}",
      //   self.extra_get(),
      //   self.cigar.right_boundry,
      //   self.extra_get() - self.cigar.right_boundry as f64,
      //   ME_LIMIT
      // );
      self.downstream(switch);
    // BUG: some values appear negative.
    // BUG: investigate the reason and consider an additional condition
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

////////////////////////////////////////////////////////////////////////////////////////////////////

fn me_get(
  raw_values: &mut RawValues,
  hm_me_collection: &Arc<Mutex<HashMap<String, f64>>>,
) {
  if let Some(me_record) =
    hm_me_collection.lock().unwrap().get(&raw_values.scaffold)
  {
    raw_values.extra = ExtraValuesEnum::MobelSize(*me_record);
  } else {
    // error!(
    //   "Mobile element: {:?} is in alignment but not in database",
    //   &raw_values.scaffold
    // );
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// purge read pairs on hashmap (record collection)
fn batch_purge(
  local_switches: &mut LocalSwtiches,
  raw_values: &mut RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
) {
  // enter block if
  // read id as changed (through read memory) indicating different batch or
  // previous read is not empty (indicating is not the first line)
  if !(raw_values.read_id.previous == raw_values.read_id.current ||
    raw_values.read_id.previous.is_empty())
  {
    // evaluate read batch
    // purge switch is true if
    // no reads have been succesfully anchored to mobile element
    // therefore previous read batch will be removed
    purge(&local_switches, &raw_values, hm_record_collection);

    // reset purge switch
    // purge switch re activates after read batch evaluation
    local_switches.activate_purge();
  }
}

fn purge(
  local_switches: &LocalSwtiches,
  raw_values: &RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
) {
  if local_switches.purge_switch {
    hm_record_collection
      .lock()
      .unwrap()
      .remove(&raw_values.read_id.previous);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// mount current data on hashmap (record collection)
fn mount(
  local_switches: &LocalSwtiches,
  raw_values: &mut RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
) -> anyResult<()> {
  // match on flag (proviral)
  // this check is much faster than using binary interpretor
  match raw_values.flag {
    // primary alignment
    proviral_flag if proviral_flag <= 255 => {
      // create new entry if not present on hashmap (record collection)
      if !hm_record_collection
        .lock()
        .unwrap()
        .contains_key(&raw_values.read_id.current)
      {
        hm_record_collection
          .lock()
          .unwrap()
          .insert(raw_values.read_id.current.clone(), MEChimericPair::new());

        // if newly inserted assign tag
        // mobile element anchor Read1
        // chromosomal anchor Read2
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
      // if already present assign tag
      // mobile element anchor Read2
      // chromosomal anchor Read1
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

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: write down tests to assert that data &
// switches are being updated properly
