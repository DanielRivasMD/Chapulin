
// standard libraries
use std::{
  fs::File,
  io::{self, prelude::*},
};

pub fn file_reader(input_file: &String) -> (BufReader, String) {
  let reader = BufReader::open(&input_file).unwrap();
  let buffer = String::new();
  return (reader, buffer)
}

pub struct BufReader {
  reader: io::BufReader<File>,
}

impl BufReader {
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
