
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use config::{Config};
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};
use clap::{ArgMatches};
use config::{File};
use anyhow::{Context};
use anyhow::Result as anyResult;
use colored::*;

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

// TODO: add compatibility for known coordinates

pub fn sv_subcmd(
  matches: &ArgMatches
) -> anyResult<()> {

  let subcmd = "SV";

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // logging
  if matches.is_present("logging") {

    let logging = matches
      .value_of("logging")
      .context(ChapulinConfigError::TODO)?;

    match logging {
      "error" => std::env::set_var("RUST_LOG", "error"),
      "warn" => std::env::set_var("RUST_LOG", "warn"),
      "info" => std::env::set_var("RUST_LOG", "info"),
      "debug" => std::env::set_var("RUST_LOG", "debug"),
      _ => (),
    }

  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // collect settings
  let verbose = matches.is_present("verbose");

  let now = SystemTime::now();
  pretty_env_logger::init();

  let config = matches.value_of("config")
    .context(ChapulinConfigError::EmptyConfigOption)?;

  if verbose {
    println!("\n{}\n{}{}", "Setting up configuration...".green(), "Configuration file read: ".blue(), config.cyan());
  }

  let mut settings = Config::default();
  settings
    .merge(File::with_name(config))
    .context(ChapulinConfigError::NoConfigFile)?;

  let settings_hm = settings.try_into::<HashMap<String, String>>()
    .context(ChapulinConfigError::ConfigHashMap{ f: config.to_string() })?;

  let directory = settings_hm.get("directory")
    .context(ChapulinConfigError::BadDirectoryVar)?;

  let output = settings_hm.get("output")
    .context(ChapulinConfigError::BadOutput)?;

  let reference_file = settings_hm.get("reference")
    .context(ChapulinConfigError::BadReferenceVar)?;

  let pair_end_reference_alignment = settings_hm.get("pair_end_reference_alignment")
    .context(ChapulinConfigError::BadPairedReferenceGenomeVar)?;

  let expected_tlen = settings_hm.get("expected_tlen")
    .context(ChapulinConfigError::TODO)?
    .parse::<i32>()
    .context(ChapulinCommonError::Parsing)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));
  let mutex_chr_assembly = Arc::new(Mutex::new(HashMap::new()));

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // reference genome module
  let crg_chr_assembly = Arc::clone(&mutex_chr_assembly);

  if verbose {
    println!("\n{}\n{}{}", "Running Reference Genome module...".green(), "Reference file read: ".blue(), reference_file.cyan());
  }

  modules::fasta_read::cache_controller::cache_controller(
    subcmd,
    directory,
    reference_file,
    crg_chr_assembly,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // structural variant module
  let c_sv_record_collection = mutex_record_collection.clone();
  let c_sv_anchor_registry = mutex_anchor_registry.clone();

  if verbose {
    println!("\n{}\n{}{}", "Running Structural Variant module...".green(), "Alignment file read: ".blue(), pair_end_reference_alignment.cyan());
  }

  modules::structural_variant::sv_controller(
    directory,
    expected_tlen,
    pair_end_reference_alignment,
    c_sv_record_collection,
    c_sv_anchor_registry,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // peak identification module
  if verbose {
    println!("\n{}\n", "Running Peak Identification module...".green());
  }

  modules::peak_identification::pi_sv_controller(
    output.to_string(),
    directory.to_string(),
    mutex_record_collection,
    mutex_anchor_registry,
    mutex_chr_assembly,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
