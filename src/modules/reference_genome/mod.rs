
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
mod reference_read;

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn ref_controller (
  directory: &String,
  reference_file: &String,
  hash_map_chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {

  // let mut hash_map_chr = HashMap::new();
  let c_hash_map_chr_assembly = hash_map_chr_assembly.clone();

  let ref_sequence = format!("{}{}", directory, reference_file);
  reference_read::reference_reader(
    ref_sequence,
    c_hash_map_chr_assembly,
  )?;

  // // output message to log
  // for (key, val) in c_hash_map_chr_assembly.iter() {
  //   println!("key: {}\nval: {:#?}", key, val);
  // }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
