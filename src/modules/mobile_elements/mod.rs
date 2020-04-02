
use std::collections::HashMap;
use crate::utils::record::*;

mod me_aligned;
mod me_registry;

pub fn me_controller(
  mut hash_map_collection: &mut HashMap < String, ReadRecord >,
) -> std::io::Result<()> {

  // define files directory
  let directory = "/Users/drivas/chapulinTest/".to_string();

  // init mobile element library hashmap
  let mut me_collection = hashmap_init();

  // load mobile element library
  let me_library = "chlSab_ltr_size.txt".to_string();
  let me_library_file = format!("{}{}", directory, me_library);
  me_registry::me_lib_loader(
    &me_library_file,
    &mut me_collection,
  ).expect(&me_library_file);

  // load mobile element aligned reads
  let prefix = "SAMN01162223_".to_string();
  let sufix = "ERV_chlSab_XV.sam".to_string();
  let me_aligned_file = format!("{}{}{}", directory, prefix, sufix);

  me_aligned::me_identificator(
    &me_aligned_file,
    &mut hash_map_collection,
    &me_collection,
  ).expect(&me_aligned_file);

  Ok(())
}
