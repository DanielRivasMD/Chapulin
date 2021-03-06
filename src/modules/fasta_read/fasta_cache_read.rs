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

pub fn read_cache(
  ref_cache: &str,
  chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {
  // load file
  let mut lines = byte_file_reader(&ref_cache)?;

  // iterate through file
  while let Some(line) = lines.next() {
    // load line into vector
    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    chr_assembly.lock().unwrap().insert(
      record_line[0].to_string(),
      record_line[1]
        .parse::<f64>()
        .context(ChapulinCommonError::Parsing)?,
    );
  }

  info!("Reading cache: {}", ref_cache);

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
