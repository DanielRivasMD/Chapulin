
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::str::{from_utf8};
use anyhow::{Context};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::functions::{
    file_reader::byte_file_reader,
  },
  utils::structures::{
    me_library::MElibrary,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  common_error::ChapulinCommonError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn me_lib_loader(
  me_lib_file: &str,
  hm_me_collection: &mut HashMap<String, MElibrary>,
) -> anyResult<()> {

  // TODO: the original scripts include tagging about LTR type. a way to indicate in a more generic manner is required
  // load file
  let mut lines = byte_file_reader(&me_lib_file)?;

  // iterate through file
  while let Some(line) = lines.next() {

    let record_line: Vec<&str> = from_utf8(line?)
      .context(ChapulinCommonError::RegistryLine)?
      .split('\t')
      .collect();

    let mobile_element_tag: String = record_line[0].parse().context(ChapulinCommonError::Parsing)?;
    let mobile_element_id: String = record_line[1].to_string();

    if ! hm_me_collection.contains_key(&mobile_element_id) {

      hm_me_collection.insert((&mobile_element_id).to_string(), MElibrary::new());

      if let Some(current_record) = hm_me_collection.get_mut(&mobile_element_id) {
        current_record.me_size = record_line[2].parse().context(ChapulinCommonError::Parsing)?;
        if mobile_element_tag == "ltr3" {
          current_record.annotations_erv.ltr3 = true;
        } else if mobile_element_tag == "ltr5" {
          current_record.annotations_erv.ltr5 = true;
        }
      }
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
