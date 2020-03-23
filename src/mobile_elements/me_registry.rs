
use std::collections::HashMap;

use crate::file_reader;
use crate::mobile_elements::me_dstruct::MELibrary;

pub fn me_lib_loader(
  me_lib_file: &String
) -> std::io::Result<()> {


  // TODO: the original scripts include tagging about LTR type. a way to indicate in a more generic manner is required

  // initiate HashMap
  let mut mobile_element_collection: HashMap<String, MELibrary> = HashMap::new();

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&me_lib_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    // TODO: load reacords onto 'mobile_element_collection'

  }

  Ok(println!("{} {}", "File read: ", &me_lib_file))
}
