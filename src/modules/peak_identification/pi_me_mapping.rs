////////////////////////////////////////////////////////////////////////////////////////////////////

// // standard libraries
use anyhow::Context;
// use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  // threshold,
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
  output: &str,
  bin_position: &BinPosition,
  // dir_strand: StrandDirection,
  // dir_registry: alias::RegistryDir,
  chr_size: f64,
  // chr_library: alias::LibraryChr,
  me_record: &alias::RecordME,
) -> alias::AnyResult {
  let psize = 25;

  // // estimate threshold
  // let cut = threshold(
  //   bin_position.count.into(),
  //   chr_size,
  //   NO_FDR as f64,
  //   &bin_position.position,
  //   psize,
  // );

  let cut = 7;

  // write results
  //////////////////////////////////////////////////

  // create file
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
  let to_write = format!("{}\n{}", read_id, record);
  // write
  fl.write_all(to_write.as_bytes()).context(
    ChapulinCommonError::WriteFile {
      f: to_write
    },
  )?;

  Ok(())
}

// Sequence identifier.
// Retrotransposon start coordinate within sequence.
// Retrotransposon end coordinate within sequence.
// Left LTR start coordinate.
// Left LTR end coordinate.
// Right LTR start coordinate.
// Right LTR end coordinate.
// % Identity between left and right LTRs (0-100).
// Left Target Site Duplication start coordinate.
// Left Target Site Duplication end coordinate.
// Right Target Site Duplication start coordinate.
// Right Target Site Duplication end coordinate.
// Polypurine Tract start coordinate.
// Polupurine Tract end coordinate.
// Strand on chromosome (+ or -).
// Percentage of purines in Polypurine Tract (0-100).
// TG motif start coordinate.
// CA motif end coordinate.

////////////////////////////////////////////////////////////////////////////////////////////////////
