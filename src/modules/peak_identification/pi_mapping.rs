
// standard libraries
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};

// crate utilities
use crate::utils::{
  read_record::ReadRecord
};
use crate::utils::chranchor_enum::ChrAnchor;

pub fn pi_identifier (
  ikey: &String,
  hm_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

  println!("{:#?}", ikey);

  // // anchor_reads = an_registry.lock().unwrap().get(&ikey);
  // println!("{:#?}", an_registry.lock().unwrap().contains_key(ikey));

  // match anchor_reads {
  //   Some(x) => {
  //     println!("{:#?}", x);
  //   },
  //   None => {
  //     println!("Not found!");
  //   },
  // }



  Ok(println!("{} {}", "Chromosome: ", &ikey))
}