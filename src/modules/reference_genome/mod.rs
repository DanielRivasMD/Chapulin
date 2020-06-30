
// standard libraries
use std::collections::HashMap;
use clap::ArgMatches;
use config::{Config, File};

// modules
mod reference_read;


pub fn ref_controller (
  matches: &ArgMatches,
) -> std::io::Result<()> {

  let mut hash_map_chr = HashMap::new();

  let config = matches.value_of("CONFIG").unwrap();
  println!("A config file was passed in: {}", config);

  let mut settings = Config::default();
    settings
      .merge(File::with_name(config)).expect("AY DIOS");

  let settings_hm = settings.try_into::<HashMap<String, String>>().unwrap();
  let directory = settings_hm.get("directory").unwrap();
  let reference_file = settings_hm.get("reference").unwrap();

  let ref_sequence = format!("{}{}", directory, reference_file);
  reference_read::reference_reader(
    ref_sequence,
    &mut hash_map_chr,
  ).expect("failed rigth here");

  // output message to log
  for (key, val) in hash_map_chr.iter() {
    println!("key: {}\nval: {:#?}", key, val);
  }

  Ok(())
}

