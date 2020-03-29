
use std::collections::HashMap;
use crate::utils::record::ReadRecord;

mod me_aligned;
mod me_registry;

pub fn me_controller(
  mut hash_map_collection: &mut HashMap < String, ReadRecord >,
) -> std::io::Result<()> {

  // // load mobile element library
  // let me_library_file = "".to_string();
  // me_registry::me_lib_loader(&me_library_file).expect(&me_library_file);

  // load mobile element aligned reads
  // let me_aligned_file = "/Users/drivas/Factorem/Chapulin/test/head.sam".to_string();
  let me_aligned_file = "/Users/drivas/Factorem/Chapulin/test/SAMN01162223_ERV_chlSab_XV.sam".to_string();
  me_aligned::me_identificator(
    &me_aligned_file,
    &mut hash_map_collection,
  ).expect(&me_aligned_file);

  Ok(())
}
