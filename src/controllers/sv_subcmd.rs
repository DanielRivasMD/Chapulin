
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};
use clap::{ArgMatches};
use config::{Config, File};
use anyhow::{Context};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
use crate::modules;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  config_error::ChapulinConfigError,
  common_error::ChapulinCommonError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn sv_subcmd(
  matches: &ArgMatches
) -> anyResult<()> {

  let now = SystemTime::now();

  println!("{:?}", now.elapsed().unwrap());

  if matches.is_present("verbose") {
    println!("Printing SV verbosely...");
  } else {
    println!("Printing SV normally...");
  }

  let config = matches.value_of("CONFIG")
    .context(ChapulinConfigError::EmptyConfigOption)?;

  println!("A config file was passed in: {}", config);

  let mut settings = Config::default();
    settings
      .merge(File::with_name(config))
      .context(ChapulinConfigError::NoConfigFile)?;

  // interpret settings into variables
  let settings_hm = settings.try_into::<HashMap<String, String>>()
    .context(ChapulinConfigError::ConfigHashMap)?;

  let pair_end_reference_alignment = settings_hm.get("pair_end_reference_alignment")
    .context(ChapulinConfigError::ConfigHashMap)?;


  let directory = settings_hm.get("directory")
    .context(ChapulinConfigError::BadDirectoryVar)?;

  let expected_tlen = settings_hm.get("expected_tlen")
    .context(ChapulinConfigError::TODO)?
    .parse::<i32>()
    .context(ChapulinCommonError::Parsing)?;

  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));

  // chromosomal loci module
  let c_sv_record_collection = mutex_record_collection.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  modules::structural_variant::sv_controller(
    directory,
    expected_tlen,
    pair_end_reference_alignment,
    c_sv_record_collection,
    mutex_anchor_registry,
  )?;

  println!("{:?}", now.elapsed().unwrap());

  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
