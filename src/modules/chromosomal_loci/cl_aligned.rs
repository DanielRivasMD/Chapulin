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
  RawValues,
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
  hm_record_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
  debug_iteration: i32,
) -> anyResult<()> {
  // load file
  let mut lines = byte_file_reader(&cl_bam_file)?;

  // create output file
  let fl_write = format!("{}.err", errata);
  let file_out =
    File::create(&fl_write).context(ChapulinCommonError::CreateFile {
      f: fl_write,
    })?;

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

    // TODO: read supplementary fields for additional information & load on
    // struct

    // mount
    mount(raw_values, &hm_record_collection, &an_registry, &file_out)?;

    if ct > debug_iteration && debug_iteration > 0 {
      break;
    }
    eval_batch(
      &mut local_switches,
      &raw_values,
      &hm_record_collection,
      &an_registry,
    );

  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new)]
struct LocalSwtiches {
  #[new(value = "true")]
  mapq: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// activation trait
trait ActivateExt {
  fn activate(&mut self);

  fn deactivate(&mut self);

  fn or_memory(
    &mut self,
    new: bool,
  );

  fn and_memory(
    &mut self,
    new: bool,
  );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// extend implement on boolean
impl ActivateExt for bool {
  fn activate(&mut self) {
    *self = true;
  }

  fn deactivate(&mut self) {
    *self = false;
  }

  fn or_memory(
    &mut self,
    new: bool,
  ) {
    *self = *self || new
  }

  fn and_memory(
    &mut self,
    new: bool,
  ) {
    *self = *self && new
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// mount current data on hashmap (record collection)
fn mount(
  raw_values: RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
  mut file_out: &File,
) -> anyResult<()> {
  // if read id is present on hashmap (record collection)
  if hm_record_collection
    .lock()
    .unwrap()
    .contains_key(&raw_values.read_id.current)
  {
    // load chromosomal anchoring data
    // check whether sequence or reverse sequence is equal
    // BUG: palindromic reads?
    load(&raw_values, hm_record_collection);

    // register
    register(raw_values, hm_record_collection, an_registry);
  } else {
    // TODO: all records are going here. investigate the reason
    file_out
      .write_all(raw_values.read_id.current.as_bytes())
      .context(ChapulinCommonError::WriteFile {
        f: raw_values.read_id.current,
      })?;
  }

  Ok(())
}

// load chromosomal anchor data on mobile element chimeric pair
fn load(
  raw_values: &RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
) {
  if let Some(current_record) = hm_record_collection
    .lock()
    .unwrap()
    .get_mut(&raw_values.read_id.current)
  {
    load!(current_record, *raw_values, read1);
    load!(current_record, *raw_values, read2);
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// register read id on scaffold
fn register(
  raw_values: RawValues,
fn eval_batch(
  local_switches: &mut LocalSwtiches,
  raw_values: &RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
) {
  // IDEA: consider tagging strand on the fly to avoid postload counting
  if anchor(&raw_values, hm_record_collection) {
    // TODO: potentially problematic
    // if raw_values.quality > MAPQ {
    // if mapq_switch {
    // if local_switches.mapq_switch {
    hm_record_collection
      .lock()
      .unwrap()
      .remove(&raw_values.read_id.current);
  } else {
    // register chromosome anchors
    if !an_registry
  if !(raw_values.read_id.previous == raw_values.read_id.current ||
    raw_values.read_id.previous.is_empty())
  {
    if local_switches.mapq {
      if raw_values.read_id.previous == "SRR556146.17" {
        println!("Removing");
        println!();
      }

      purge(&raw_values, hm_record_collection);
    } else {
      if raw_values.read_id.previous == "SRR556146.17" {
        println!("Registering");
        println!();
      }

      register(an_registry, raw_values);
      local_switches.mapq.activate();
    }
  }

  // memory
  local_switches
    .mapq
    .and_memory(anchor(raw_values, hm_record_collection));
}

      .lock()
      .unwrap()
      .contains_key(&raw_values.scaffold)
    {
      // clone scaffold value here
      an_registry
        .lock()
        .unwrap()
        .insert(raw_values.scaffold.clone(), Vec::new());
    }

    if let Some(current_chr) =
      an_registry.lock().unwrap().get_mut(&raw_values.scaffold)
    {
      if !current_chr.contains(&raw_values.read_id.current) {
        // observe that value of the current read is moved here
        current_chr.push(raw_values.read_id.current)
      }
    }
  }
}

// read chromosomal anchor enum
fn anchor(
  raw_values: &RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
) -> bool {
  let mut switch_out = false;
  if let Some(current_record) = hm_record_collection
    .lock()
    .unwrap()
    .get(&raw_values.read_id.current)
  {
    match current_record.chranch {
      ChrAnchorEnum::Read1 => switch_out = mapq!(current_record, read1),
      ChrAnchorEnum::Read2 => switch_out = mapq!(current_record, read2),
      _ => (),
    };
  }
  switch_out
}

////////////////////////////////////////////////////////////////////////////////////////////////////
