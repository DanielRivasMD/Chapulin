////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use bytelines::ByteLinesReader;
use std::fs::File;
use std::io::BufReader;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn byte_file_reader(input_file: &str) -> alias::AnyBufferResult {
  let file =
    File::open(&input_file).context(ChapulinCommonError::ReadFile {
      f: input_file.to_string(),
    })?;

  let reader = BufReader::new(file);

  let lines = reader.byte_lines();
  Ok(lines)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
