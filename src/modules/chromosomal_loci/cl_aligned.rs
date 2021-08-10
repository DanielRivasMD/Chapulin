////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: heavily comment
// TODO: since modules are functional, consider implementing error handlers per
// function. this assumes that error handlers can be efficiently tested

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

    // // debugger counter
    // ct += 1;
    // if ct % 10000 == 0 {
    //   // println!("{}", ct);
    // }

    // SAM line values updated at each iteration
    // observe that raw values holds read control
    // for keeping the state of read batch
    raw_values.update(record_line)?;

    if raw_values.read_id.current == "SRR556146.17" {
      println!("{:?}", raw_values.sequence);
      println!("{:?}", raw_values.quality);
    }
    // TODO: read supplementary fields for additional information & load on
    // struct

    // mount
    mount(&raw_values, &hm_record_collection, &file_out)?;

    eval_batch(
      &mut local_switches,
      &raw_values,
      &hm_record_collection,
      &an_registry,
    );

    // remember previous read
    raw_values.read_id.read_memory();

    // println!("Iteration: {} -> {}", ct, local_switches.mapq);
    // if ct > debug_iteration && debug_iteration > 0 {
    //   //   // println!("{:#?}", hm_record_collection);
    //   break;
    // }
  }

  // if let Some(cr) = hm_record_collection.lock().unwrap().get("SRR556146.17")
  // {   println!("Read1: {:#?}", cr.read1.sequence);
  //   println!("Read1: {:#?}", cr.read1.me_read);
  //   println!("Read2: {:#?}", cr.read2.sequence);
  //   println!("Read2: {:#?}", cr.read2.chr_read);
  // }
  // // println!("{:#?}", hm_record_collection);

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
  // local_switches: &LocalSwtiches,
  raw_values: &RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  // an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
  mut _file_out: &File,
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

  // // register
  // register(
  //   &mut local_switches,
  //   raw_values,
  //   hm_record_collection,
  //   an_registry,
  // );
  } else {
    // TODO: all records are going here. investigate the reason
    // file_out
    //   .write_all(raw_values.read_id.current.as_bytes())
    //   .context(ChapulinCommonError::WriteFile {
    //     f: raw_values.read_id.current.clone(),
    //   })?;
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
fn eval_batch(
  local_switches: &mut LocalSwtiches,
  raw_values: &RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
) {
  // let _ = anchor(raw_values, hm_record_collection);
  // local_switches.mapq =
  //   local_switches.mapq || anchor(raw_values, hm_record_collection);

  // evaluate read batch
  // batch_purge(local_switches, raw_values, hm_record_collection);

  // TODO: why are the reads not in order. also, this function should account
  // for that fact since it must support single-end alignments as well
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

// if local_switches.mapq {
//   purge()
// }

// IDEA: consider tagging strand on the fly to avoid postload counting
// BUG: this switch must contain memory, otherwise it'll delete all read2
// if anchor(&raw_values, hm_record_collection) {
//   if raw_values.read_id.current == "SRR556146.17" {
//     println!("Removing");
//   }
//   hm_record_collection
//     .lock()
//     .unwrap()
//     .remove(&raw_values.read_id.current);
// } else {
//   if raw_values.read_id.current == "SRR556146.17" {
//     println!("Registering");
//   }
// register chromosome anchors

fn register(
  an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
  raw_values: &RawValues,
) {
  if !an_registry
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
    // verify whether vector contains entry
    if !current_chr.contains(&raw_values.read_id.previous) {
      // observe that value of the current read is moved here
      current_chr.push(raw_values.read_id.previous.clone())
    }
  }
}

// read chromosomal anchor enum
// TODO: considering that reads are previously tagged for chr anchor,
// remembering the state is not a problem because all filtering can be
// done on the current read.
fn anchor(
  raw_values: &RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
) -> bool {
  let mut switch_out = true;
  if let Some(current_record) = hm_record_collection
    .lock()
    .unwrap()
    .get(&raw_values.read_id.current)
  {
    // println!("{:#?}", current_record);
    match current_record.chranch {
      ChrAnchorEnum::Read1 => {
        switch_out = mapq!(current_record, read1);
      }
      ChrAnchorEnum::Read2 => {
        switch_out = mapq!(current_record, read1);
        println!("Inside Match");
        println!("{:?}", current_record.read1.chr_read.is_empty());
        println!("{:?}", current_record.read1.chr_read[0].mapq < MAPQ);
        println!("{:?}", switch_out);
      }
      _ => (),
    };
  }

  if raw_values.read_id.current == "SRR556146.17" {
    println!("Inside Match");
    //   println!();
    //   println!("{:#?}", raw_values);
    //   println!("Switch: {:?}", switch_out);
  }
  switch_out
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// // purge read pairs on hashmap (record collection)
// fn batch_purge(
//   local_switches: &mut LocalSwtiches,
//   raw_values: &RawValues,
//   hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
// ) {
//   // enter block if
//   // read id as changed (through read memory) indicating different batch or
//   // previous read is not empty (indicating is not the first line)
//   if !(raw_values.read_id.previous == raw_values.read_id.current ||
//     raw_values.read_id.previous.is_empty())
//   {
//     // evaluate read batch
//     // purge switch is true if
//     // no reads have been succesfully anchored to mobile element
//     // therefore previous read batch will be removed
//     purge(local_switches, raw_values, hm_record_collection);

//     // reset purge switch
//     // purge switch re activates after read batch evaluation
//     local_switches.mapq.deactivate();
//   }
// }

fn purge(
  // local_switches: &LocalSwtiches,
  raw_values: &RawValues,
  hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
) {
  // if local_switches.mapq {
  hm_record_collection
    .lock()
    .unwrap()
    .remove(&raw_values.read_id.previous);
  // }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
