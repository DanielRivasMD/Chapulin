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

// crate utilities
use crate::utils::io::file_reader::byte_file_reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn fasta_reader(
  ref_seq: &str,
  fasta_record: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {
  let mut current_chr = String::new();
  let mut current_len = 0.;

  let mut lines = byte_file_reader(&ref_seq)?;
  while let Some(line) = lines.next() {
    let record_line =
      from_utf8(&line?).context(ChapulinCommonError::RegistryLine)?;

    if record_line.starts_with('>') {
      if !current_chr.is_empty() {
        fasta_record
          .lock()
          .unwrap()
          .insert(current_chr, current_len);
        current_len = 0.;
      }
      let record_entry: Vec<&str> = record_line.trim().split(' ').collect();
      current_chr = record_entry[0].replace('>', "");
    } else {
      current_len += record_line.len() as f64;
    }
  }
  fasta_record
    .lock()
    .unwrap()
    .insert(current_chr, current_len as f64);

  info!("Reading fasta: {}", ref_seq);

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
