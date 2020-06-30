use crate::utils::file_reader::byte_file_reader;
use std::str::from_utf8;
use std::collections::HashMap;
// use std::thread::current;

pub fn reference_reader(
  ref_seq: String,
  chr_registry: &mut HashMap<String, i64>,
) -> std::io::Result<()> {

  let mut current_chr = String::new();

  let mut lines = byte_file_reader(&ref_seq);
  while let Some(line) = lines.next() {

    let record_line = from_utf8(&line?).unwrap();

    if record_line.starts_with('>') {
      let record_entry: Vec<&str> = record_line.trim().split(" ").collect();
      current_chr = record_entry[0].replace(">", "");
    } else {
      chr_registry.insert(current_chr, record_line.len() as i64);
      current_chr = String::new();
    }
  }
  Ok(())
}