
use std::collections::HashMap;
use crate::utils::record::ReadRecord;

mod cl_aligned;

pub fn cl_controller(
    mut hash_map_collection: &mut HashMap < String, ReadRecord >,
) -> std::io::Result<()> {

  // load reference chromosome aligned reads
  let cl_aligned_file = "/Users/drivas/Factorem/Chapulin/test/SAMN01162223.sorted.RemoveSupplementary.MarkDuplicates.sam".to_string();
  cl_aligned::cl_mapper(
    &cl_aligned_file,
    &mut hash_map_collection,
  ).expect(&cl_aligned_file);

  Ok(())
}
