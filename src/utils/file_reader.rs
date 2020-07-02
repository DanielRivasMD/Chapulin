
// standard libraries
use std::{
  fs::File,
  io::{
    self,
    BufReader,
    prelude::*
  },
};

use bytelines::{
  ByteLines,
  ByteLinesReader,
};


pub fn byte_file_reader(
  input_file: &String,
) -> ByteLines<BufReader<File>> {

  let file = File::open(&input_file)
    .expect(format!("\n\nProblem opening file:\n\n{}\n\nPossibly file does not exist\n\n", &input_file).as_str());
  let reader = BufReader::new(file);

  let lines = reader.byte_lines();
  return lines
}

pub fn buff_file_reader(input_file: &String) -> (CustBufReader, String) {
  let reader = CustBufReader::open(&input_file)
    .expect(format!("\n\nProblem opening file:\n\n{}\n\nPossibly file does not exist\n\n", &input_file).as_str());
  let buffer = String::new();
  return (reader, buffer)
}

pub struct CustBufReader {
  reader: io::BufReader<File>,
}

impl CustBufReader {
  pub fn open(
    path: impl AsRef<std::path::Path>
  ) -> io::Result<Self> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    Ok(Self { reader })
  }

  pub fn read_line<'buf>(
    &mut self,
    buffer: &'buf mut String,
  ) -> Option<io::Result<&'buf mut String>> {
    buffer.clear();

    self.reader
      .read_line(buffer)
      .map(|u| if u == 0 { None } else { Some(buffer) })
      .transpose()
  }
}
