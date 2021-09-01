////////////////////////////////////////////////////////////////////////////////////////////////////

// // standard libraries
use anyhow::Context;
// use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  threshold,
  BinPosition,
  MEChimericPair,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::settings::constants::NO_FDR;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pi_me_identifier(
  ikey: &str,
  output: &str,
  _errata: &str,
  chr_registry: alias::RegistryChr,
  me_library: alias::LibraryME,
  me_record: alias::RecordME,
  bin_position: &BinPosition,
  // dir_strand: StrandDirection,
  // dir_registry: alias::RegistryDir,
  chr_size: f64,
  // chr_library: alias::LibraryChr,
  me_record: &alias::RecordME,
) -> alias::AnyResult {
  // let mut chr_position_hm = HashMap::new();
  // let chr_size = *me_library.lock().unwrap().get(ikey).unwrap();

  let psize = 25;

  // estimate threshold
  let cut = threshold(
    bin_position.count.into(),
    chr_size,
    NO_FDR as f64,
    &bin_position.position,
    psize,
  );

  // write results
  //////////////////////////////////////////////////

  // create file
  // let fl_write = format!("{}{}.csv", output, ikey);
  // let mut fl =
  //   File::create(&fl_write).context(ChapulinCommonError::CreateFile {
  //     f: fl_write,
  //   })?;
  let mut fl =
    File::create(&output).context(ChapulinCommonError::CreateFile {
      f: output.to_string(),
    })?;

  // select records
  select(&mut fl, cut, bin_position, me_record)?;

  //   // TODO: tag orientation to reduce elements to iterate on
  //   // TODO: check for non-oriented mobels
  //   // TODO: implement a threshold selector

  //   // TODO: memotization

  //     // TODO: verify read pairs some are empty
  //     // TODO: verify read passing map quality
  //     // TODO: verify reads no counted twice
  //     // TODO: format output => possibly 1) raw 2) postgreSQL

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn select(
  fl: &mut File,
  cut: usize,
  bin_position: &BinPosition,
  me_record: &alias::RecordME,
) -> alias::AnyResult {
  for (_position, reads) in bin_position.position.iter() {
    if reads.len() > cut {
      for read_id in reads.iter() {
        if let Some(record) = me_record.lock().unwrap().get(read_id) {
          write(fl, record, read_id)?;
        }
      }
    }
  }

  Ok(())
}

fn write(
  fl: &mut File,
  record: &MEChimericPair,
  read_id: &str,
) -> alias::AnyResult {
  // format line
  let to_write = format!("{}\t{}\n", read_id, record.read1.me_read[0].position);
  // write
  fl.write_all(to_write.as_bytes()).context(
    ChapulinCommonError::WriteFile {
      f: to_write
    },
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
