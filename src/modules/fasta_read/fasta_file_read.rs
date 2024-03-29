////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use std::str::from_utf8;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::io::file_reader::byte_file_reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Read fasta file to obtain sequence lenghts.
pub fn fasta_read(
  ref_seq: &str,
  fasta_record: alias::LibraryME,
) -> alias::AnyResult {
  // initalize mutable values
  let mut current_chr = String::new();
  let mut current_len = 0.;

  // loop over file lines
  let mut lines = byte_file_reader(&ref_seq)?;
  while let Some(line) = lines.next() {
    // read line
    let record_line =
      from_utf8(&line?).context(ChapulinCommonError::RegistryLine)?;

    // identify fasta tags
    if record_line.starts_with('>') {
      // mount previous record on arch mutex hashmap
      if !current_chr.is_empty() {
        fasta_record
          .lock()
          .unwrap()
          .insert(current_chr, current_len);
        // reset line length
        current_len = 0.;
      }
      // format scaffold / chromosome name
      let record_entry: Vec<&str> = record_line.trim().split(' ').collect();
      current_chr = record_entry[0].replace('>', "");
    } else {
      // collect lenght of non-fasta-tagged lines
      current_len += record_line.len() as f64;
    }
  }

  // mount last record on arch mutex hashmap
  fasta_record
    .lock()
    .unwrap()
    .insert(current_chr, current_len as f64);

  // log
  info!("Reading fasta: {}", ref_seq);

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
