
// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};
use clap::{ArgMatches};
use config::{Config, File};

// modules
use crate::modules;


pub fn sv_subcmd(
  matches: &ArgMatches
) -> std::io::Result<()> {

  let now = SystemTime::now();

  println!("{:?}", now.elapsed().unwrap());

  if matches.is_present("verbose") {
    println!("Printing SV verbosely...");
  } else {
    println!("Printing SV normally...");
  }

  let config = matches.value_of("CONFIG").unwrap();
  println!("A config file was passed in: {}", config);

  let mut settings = Config::default();
    settings
      .merge(File::with_name(config)).unwrap();

  // interpret settings into variables
  let settings_hm = settings.try_into::<HashMap<String, String>>().unwrap();

  let pair_end_reference_alignment = settings_hm.get("pair_end_reference_alignment").unwrap();

  let directory = settings_hm.get("directory").unwrap();

  let expected_tlen = settings_hm.get("expected_tlen").unwrap().parse::<i32>().unwrap();
  // let sv_align = settings_hm.get("reference_genome_alignment").unwrap();

  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));

  // chromosomal loci module
  let c_sv_record_collection = mutex_record_collection.clone();
  let c_sv_anchor_registry = mutex_anchor_registry.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  modules::structural_variant::sv_controller(
    directory,
    expected_tlen,
    pair_end_reference_alignment,
    c_sv_record_collection,
    c_sv_anchor_registry,
  )?;

  println!("{:?}", now.elapsed().unwrap());

  // let since_the_epoch = now
  //   .duration_since(UNIX_EPOCH)
  //   .expect("Time went backwards");
  // println!("{:?}", since_the_epoch);

  // match now.elapsed() {
  //   Ok(elapsed) => {
  //     println!("{} secs", elapsed.as_secs_f64());
  //   }
  //
  //   Err(e) => {
  //     // an error occurred!
  //     println!("Error: {:?}", e);
  //   }
  // }

  // // peak identification module
  // let c_pi_record_collection = mutex_record_collection.clone();
  // let c_pi_anchor_registry = mutex_anchor_registry.clone();
  // println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());
  //
  // modules::peak_identification::pi_controller(
  //   c_pi_record_collection,
  //   c_pi_anchor_registry,
  // )?;

  // output message to log
  for (key, val) in mutex_record_collection.lock().unwrap().iter() {
    println!("key: {}\nval: {:#?}", key, val);
  }

  println!("{:?}", now.elapsed().unwrap());

  // match now.elapsed() {
  //   Ok(elapsed) => {
  //     println!("{} secs", elapsed.as_secs_f64());
  //   }
  //
  //   Err(e) => {
  //     // an error occurred!
  //     println!("Error: {:?}", e);
  //   }
  // }

  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  // match now.elapsed() {
  //   Ok(elapsed) => {
  //     println!("{} secs", elapsed.as_secs_f64());
  //   }
  //
  //   Err(e) => {
  //     // an error occurred!
  //     println!("Error: {:?}", e);
  //   }
  // }

  Ok(())
}
