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
  let mut file_out =
    File::create(&fl_write).context(ChapulinCommonError::CreateFile {
      f: fl_write,
    })?;

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

    // // debugger counter
    // ct += 1;
    // if ct % 10000 == 0 {
    //   // println!("{}", ct);
    // }

    // SAM line values declared at each iteration
    let raw_values = RawValues::load(record_line)?;

    // TODO: read supplementary fields for additional information & load on
    // struct

    if raw_values.read_id.current == "SRR556146.17" {
      println!("{:?}", raw_values.sequence);
      println!("{:?}", raw_values.quality);
    }

    // mount
    raw_values.mount(&hm_record_collection, &an_registry, &mut file_out)?;

    if ct > debug_iteration && debug_iteration > 0 {
      break;
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

trait MountExt {
  fn mount(
    self,
    hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
    an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
    file_out: &mut File,
  ) -> anyResult<()>;

  fn load(
    &self,
    hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MountExt for RawValues {
  // mount current data on hashmap (record collection)
  fn mount(
    self,
    hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
    an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
    file_out: &mut File,
  ) -> anyResult<()> {
    // if read id is present on hashmap (record collection)
    if hm_record_collection
      .lock()
      .unwrap()
      .contains_key(&self.read_id.current)
    {
      // load chromosomal anchoring data
      // check whether sequence or reverse sequence is equal
      // BUG: palindromic reads?
      self.load(hm_record_collection);

      // register
      self.register(hm_record_collection, an_registry);
    } else {
      // TODO: all records are going here. investigate the reason
      file_out
        .write_all(self.read_id.current.as_bytes())
        .context(ChapulinCommonError::WriteFile {
          f: self.read_id.current,
        })?;
    }

    Ok(())
  }

  // load chromosomal anchor data on mobile element chimeric pair
  fn load(
    &self,
    hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  ) {
    if let Some(current_record) = hm_record_collection
      .lock()
      .unwrap()
      .get_mut(&self.read_id.current)
    {
      load!( chromosomal => current_record; *self; read1 );
      load!( chromosomal => current_record; *self; read2 );
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

trait RegisterExt {
  fn register(
    self,
    hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
    an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
  );

  fn anchor(
    &self,
    hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  ) -> bool;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: why are the reads not in order. also, this function should account
// for that fact since it must support single-end alignments as well
// IDEA: consider tagging strand on the fly to avoid postload counting
// BUG: this switch must contain memory, otherwise it'll delete all read2
impl RegisterExt for RawValues {
  // register read id on scaffold
  fn register(
    self,
    hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
    an_registry: &Arc<Mutex<HashMap<String, Vec<String>>>>,
  ) {
    if self.anchor(hm_record_collection) {
      if self.read_id.current == "SRR556146.17" {
        println!("Removing");
        println!();
      }
      hm_record_collection
        .lock()
        .unwrap()
        .remove(&self.read_id.current);
    } else {
      if self.read_id.current == "SRR556146.17" {
        println!("Registering");
        println!();
      }
      // register chromosome anchors
      if !an_registry.lock().unwrap().contains_key(&self.scaffold) {
        // clone scaffold value here
        an_registry
          .lock()
          .unwrap()
          .insert(self.scaffold.clone(), Vec::new());
      }

      if let Some(current_chr) =
        an_registry.lock().unwrap().get_mut(&self.scaffold)
      {
        // verify whether vector contains entry
        if !current_chr.contains(&self.read_id.current) {
          // observe that value of the current read is moved here
          current_chr.push(self.read_id.current)
        }
      }
    }
  }

  fn anchor(
    &self,
    hm_record_collection: &Arc<Mutex<HashMap<String, MEChimericPair>>>,
  ) -> bool {
    let mut switch_out = true;
    if let Some(current_record) = hm_record_collection
      .lock()
      .unwrap()
      .get(&self.read_id.current)
    {
      // println!("{:#?}", current_record);
      match current_record.chranch {
        ChrAnchorEnum::Read1 => {
          switch_out = mapq!(current_record, read1);
        }
        ChrAnchorEnum::Read2 => {
          switch_out = mapq!(current_record, read2);
          println!("Inside Match");
          println!("{:?}", current_record.read1.chr_read.is_empty());
          println!("{:?}", current_record.read1.chr_read[0].mapq < MAPQ);
          println!("{:?}", switch_out);
        }
        _ => (),
      };
    }

    if self.read_id.current == "SRR556146.17" {
      println!("Inside Match");
      //   println!();
      //   println!("{:#?}", self);
      //   println!("Switch: {:?}", switch_out);
    }
    switch_out
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
