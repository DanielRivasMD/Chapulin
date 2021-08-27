////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use std::str::from_utf8;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  ExtraValuesEnum,
  MEAnchor,
  MEChimericPair,
  OrientationEnum,
  RawValues,
  TagME,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::io::file_reader::byte_file_reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate features
use crate::ActivateExt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// module features
use super::LocalSwtiches;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: extract features from fasta other than sequence length
/// Identify mobile elements.
pub fn me_identificator(
  me_bam_file: &str,
  hm_me_collection: alias::LibraryME,
  hm_record_collection: alias::RecordME,
  debug_iteration: i32,
) -> alias::AnyResult {
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

    // omit incomplete records
    if record_line.len() < 11 {
      continue;
    }

    // debugger counter
    ct += 1;

    // SAM line values updated at each iteration
    // observe that raw values holds read control
    // for keeping the state of read batch
    raw_values.update(record_line)?;

    // purge read pairs on hashmap (record collection)
    // evaluate batch, based on previous read, immediately after
    // loading new values into raw values
    raw_values.batch_purge(&mut local_switches, &hm_record_collection);

    // retrieve mobile element library records
    raw_values.library_get(&hm_me_collection);

    // tagging mobel anchor
    // switches get updated by local switches methods
    raw_values.mobel_tag(&mut local_switches);

    // mount current data on hashmap (record collection)
    raw_values.mount(&hm_record_collection)?;

    // reset orientation
    raw_values.reset_orientation();

    // remember previous read
    raw_values.read_id.read_memory();

    if ct > debug_iteration && debug_iteration > 0 {
      // println!("{:#?}", hm_record_collection);
      break;
    }
  }

  // evaluate at end of file
  //////////////////////////////////////////////////

  // tag
  raw_values.tag(&hm_record_collection);

  // purge
  raw_values.purge(&local_switches, &hm_record_collection);
  // println!("{:?}", hm_record_collection.lock().unwrap().keys());

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// extend functionality of raw values locally
trait MEAnchorExt {
  fn mobel_tag(
    &mut self,
    switch: &mut LocalSwtiches,
  );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// extend implementation on raw values
impl MEAnchorExt for RawValues {
  // since read orientation can be calculated with only
  // on current values on the fly through function
  fn mobel_tag(
    &mut self,
    switch: &mut LocalSwtiches,
  ) {
    // tag mobile element
    self.tag();
    // modify switches accordingly
    if self.orientation != OrientationEnum::None {
      switch.switches();
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// extend functionality of raw values locally
trait LibraryExt {
  fn library_get(
    &mut self,
    hm_record_collection: &alias::LibraryME,
  );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// mobile element library
impl LibraryExt for RawValues {
  // TODO: transform this function into extend implementation
  // collect mobile element from library & mount it on raw values extra enum
  fn library_get(
    &mut self,
    hm_me_collection: &alias::LibraryME,
  ) {
    if let Some(me_record) =
      hm_me_collection.lock().unwrap().get(&self.scaffold)
    {
      self.extra = ExtraValuesEnum::MobelSize(*me_record);
    } else {
      // error!(
      //   "Mobile element: {:?} is in alignment but not in database",
      //   &self.scaffold
      // );
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

trait TagExt {
  fn tag(
    &self,
    hm_record_collection: &alias::RecordME,
  );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl TagExt for RawValues {
  fn tag(
    &self,
    hm_record_collection: &alias::RecordME,
  ) {
    // tag chromosomal anchor by iterating on all mobile element anchor
    // recorded
    if let Some(current_record) = hm_record_collection
      .lock()
      .unwrap()
      .get_mut(&self.read_id.previous)
    {
      current_record.tag();
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

trait PurgeExt {
  fn batch_purge(
    &self,
    local_switches: &mut LocalSwtiches,
    hm_record_collection: &alias::RecordME,
  );

  fn purge(
    &self,
    local_switches: &LocalSwtiches,
    hm_record_collection: &alias::RecordME,
  );
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl PurgeExt for RawValues {
  // purge read pairs on hashmap (record collection)
  fn batch_purge(
    &self,
    local_switches: &mut LocalSwtiches,
    hm_record_collection: &alias::RecordME,
  ) {
    // enter block if
    // read id as changed (through read memory) indicating different batch or
    // previous read is not empty (indicating is not the first line)
    if !(self.read_id.previous == self.read_id.current ||
      self.read_id.previous.is_empty())
    {
      // tag
      self.tag(hm_record_collection);

      // evaluate read batch
      // purge switch is true if
      // no reads have been succesfully anchored to mobile element
      // therefore previous read batch will be removed
      self.purge(local_switches, hm_record_collection);

      // reset purge switch
      // purge switch re activates after read batch evaluation
      local_switches.purge.activate();
    }
  }

  // TODO: consider using drain_filter from HashMap to purge records
  fn purge(
    &self,
    local_switches: &LocalSwtiches,
    hm_record_collection: &alias::RecordME,
  ) {
    if local_switches.purge {
      hm_record_collection
        .lock()
        .unwrap()
        .remove(&self.read_id.previous);
    }
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

trait MountExt {
  fn mount(
    &self,
    hm_record_collection: &alias::RecordME,
  ) -> alias::AnyResult;
}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MountExt for RawValues {
  // TODO: perhaps add switches to control where records are assigned?
  // mount current data on hashmap (record collection)
  fn mount(
    &self,
    hm_record_collection: &alias::RecordME,
  ) -> alias::AnyResult {
    // match on flag (proviral)
    // this check is much faster than using binary interpretor
    match self.flag {
      // primary alignment
      proviral_flag if proviral_flag <= 255 => {
        // create new entry if not present on hashmap (record collection)
        if !hm_record_collection
          .lock()
          .unwrap()
          .contains_key(&self.read_id.current)
        {
          hm_record_collection
            .lock()
            .unwrap()
            .insert(self.read_id.current.clone(), MEChimericPair::new());

          // if newly inserted assign tag
          // mobile element anchor Read1
          // chromosomal anchor Read2
          if let Some(current_record) = hm_record_collection
            .lock()
            .unwrap()
            .get_mut(&self.read_id.current)
          {
            load!( mobile element |> current_record; self; read1 );
          }
        // if already present assign tag
        // mobile element anchor Read2
        // chromosomal anchor Read1
        } else if let Some(current_record) = hm_record_collection
          .lock()
          .unwrap()
          .get_mut(&self.read_id.current)
        {
          load!( mobile element |> current_record; self; read2 );
        }
      }

      // secondary alignment
      proviral_flag if proviral_flag >= 256 => {
        if let Some(current_record) = hm_record_collection
          .lock()
          .unwrap()
          .get_mut(&self.read_id.current)
        {
          // if sequence field is empty insert indicates no primary alignment
          // has been filled on read 2 this assumes secondary
          // alignments are ordered
          if current_record.read2.sequence.is_empty() {
            load!( mobile element |> current_record; self; read1 );
          } else {
            load!( mobile element |> current_record; self; read2 );
          }
        }
      }

      _ => (),
    }

    Ok(())
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
