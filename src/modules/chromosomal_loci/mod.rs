
// standard libraries
use std::collections::HashMap;

// crate utilities
use crate::utils::read_record::ReadRecord;

// modules
mod cl_aligned;

pub fn cl_controller(
  mut hash_map_collection: &mut HashMap<String, ReadRecord>,
  mut hash_map_anchor: &mut HashMap<String, Vec<String>>,
) -> std::io::Result<()> {

  // load reference chromosome aligned reads
  for i in 1..3 {

    let directory = "/Users/drivas/chapulinTest/".to_string();
    let prefix = "SAMN01162223_R".to_string();
    let sufix = ".sorted.sam".to_string();
    let cl_aligned_file = format!("{}{}{}{}", directory, prefix, i, sufix);

    cl_aligned::cl_mapper(
      &cl_aligned_file,
      &mut hash_map_collection,
      &mut hash_map_anchor,
    ).expect(&cl_aligned_file);
  }

  Ok(())
}
