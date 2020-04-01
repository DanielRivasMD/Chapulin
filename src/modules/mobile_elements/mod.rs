
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
  let directory = "/Users/drivas/chapulinTest/".to_string();
  let prefix = "SAMN01162223_".to_string();
  let sufix = "ERV_chlSab_XV.sam".to_string();
  let me_aligned_file = format!("{}{}{}", directory, prefix, sufix);

  me_aligned::me_identificator(
    &me_aligned_file,
    &mut hash_map_collection,
  ).expect(&me_aligned_file);

  Ok(())
}
