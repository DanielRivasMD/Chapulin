
// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::str::{from_utf8};

// crate utilities
use crate::{
  utils::{
    file_reader::byte_file_reader,
  }
};


pub fn reference_reader(
  ref_seq: String,
  chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> std::io::Result<()> {

  let mut current_chr = String::new();

  let mut lines = byte_file_reader(&ref_seq);
  while let Some(line) = lines.next() {

    let record_line = from_utf8(&line?).unwrap();

    if record_line.starts_with('>') {
      let record_entry: Vec<&str> = record_line.trim().split(" ").collect();
      current_chr = record_entry[0].replace(">", "");
    } else {
      chr_assembly.lock().unwrap().insert(current_chr, record_line.len() as f64);
      current_chr = String::new();
    }
  }
  Ok(())
}