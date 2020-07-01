
// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};

// modules
mod reference_read;


pub fn ref_controller (
  directory: &String,
  reference_file: &String,
  hash_map_chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> std::io::Result<()> {

  // let mut hash_map_chr = HashMap::new();
  let c_hash_map_chr_assembly = hash_map_chr_assembly.clone();

  // let config = matches.value_of("CONFIG").unwrap();
  // println!("A config file was passed in: {}", config);
  //
  // let mut settings = Config::default();
  //   settings
  //     .merge(File::with_name(config)).expect("AY DIOS");
  //
  // let settings_hm = settings.try_into::<HashMap<String, String>>().unwrap();
  // let directory = settings_hm.get("directory").unwrap();
  // let reference_file = settings_hm.get("reference").unwrap();

  let ref_sequence = format!("{}{}", directory, reference_file);
  reference_read::reference_reader(
    ref_sequence,
    c_hash_map_chr_assembly,
  ).expect("failed rigth here");

  // output message to log
  for (key, val) in hash_map_chr.iter() {
    println!("key: {}\nval: {:#?}", key, val);
  }

  Ok(())
}

