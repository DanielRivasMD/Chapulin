
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::fs::{File};
use std::io::{self, BufReader, {prelude::*}};
use bytelines::{ByteLines, ByteLinesReader};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn byte_file_reader(
  input_file: &String,
) -> ByteLines<BufReader<File>> {

  let file = File::open(&input_file)
    .expect(format!("\n\nProblem opening file:\n\n{}\n\nPossibly file does not exist\n\n", &input_file).as_str());
  let reader = BufReader::new(file);

  let lines = reader.byte_lines();
  return lines
}

////////////////////////////////////////////////////////////////////////////////////////////////////
