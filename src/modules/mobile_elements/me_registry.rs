
// standard libraries
use std::collections::HashMap;

// crate utilities
use crate::utils::{
  file_reader,
  me_library::{
    MElibrary,
  }
};

pub fn me_lib_loader(
  me_lib_file: &String,
  hm_me_collection: &mut HashMap<String, MElibrary>,
) -> std::io::Result<()> {

  // TODO: the original scripts include tagging about LTR type. a way to indicate in a more generic manner is required
  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&me_lib_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    let mobile_element_tag: String = record_line[0].parse().unwrap();
    let mobile_element_id: String = record_line[1].to_string();

    if ! hm_me_collection.contains_key(&mobile_element_id) {

      hm_me_collection.insert((&mobile_element_id).to_string(), MElibrary::new());

      if let Some(current_record) = hm_me_collection.get_mut(&mobile_element_id) {
        current_record.me_size = record_line[2].parse().unwrap();
        if mobile_element_tag == "ltr3".to_string() {
          current_record.annotations_erv.ltr3 = true;
        } else if mobile_element_tag == "ltr5".to_string() {
          current_record.annotations_erv.ltr5 = true;
        }
      }
    }
  }

  Ok(println!("{} {}", "File read: ", &me_lib_file))
}
