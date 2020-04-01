
use std::collections::HashMap;
use crate::utils::record::ReadRecord;

mod cl_aligned;

pub fn cl_controller(
    mut hash_map_collection: &mut HashMap < String, ReadRecord >,
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
    ).expect(&cl_aligned_file);
  }

  Ok(())
}
