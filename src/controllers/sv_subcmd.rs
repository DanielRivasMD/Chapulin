
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


  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // collect settings
  let verbose = matches.is_present("verbose");

  let now = SystemTime::now();

  let config = matches.value_of("CONFIG")
    .context(ChapulinConfigError::EmptyConfigOption)?;

  if verbose {
    println!("Configuration file read: {}\n", config);
  }

  let mut settings = Config::default();
  settings
    .merge(File::with_name(config))
    .context(ChapulinConfigError::NoConfigFile)?;

  // interpret settings into variables
  let settings_hm = settings.try_into::<HashMap<String, String>>()
    .context(ChapulinConfigError::ConfigHashMap)?;

  let directory = settings_hm.get("directory")
    .context(ChapulinConfigError::BadDirectoryVar)?;

  let pair_end_reference_alignment = settings_hm.get("pair_end_reference_alignment")
    .context(ChapulinConfigError::ConfigHashMap)?;

  let expected_tlen = settings_hm.get("expected_tlen")
    .context(ChapulinConfigError::TODO)?
    .parse::<i32>()
    .context(ChapulinCommonError::Parsing)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // structural variant module
  let c_sv_record_collection = mutex_record_collection.clone();
  let c_sv_anchor_registry = mutex_anchor_registry.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  if verbose {
    println!("\nRunning Structural Variant module...");
    println!("Alignment file read: {}\n", pair_end_reference_alignment);
  }

  modules::structural_variant::sv_controller(
    directory,
    expected_tlen,
    pair_end_reference_alignment,
    c_sv_record_collection,
    c_sv_anchor_registry,
  )?;

  println!("{:?}", now.elapsed().unwrap());

    c_sv_record_collection,
    mutex_anchor_registry,
  )?;

  println!("{:?}", now.elapsed().unwrap());

  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
