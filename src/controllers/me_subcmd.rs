
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use config::Config;
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
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn me_subcmd(
  matches: &ArgMatches
) -> anyResult<()> {


  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // logging
  if matches.is_present("LOGGING") {

    let logging = matches
      .value_of("LOGGING")
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

  let config = matches.value_of("CONFIG")
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

  let me_library_file = settings_hm.get("mobile_element_library")
    .context(ChapulinConfigError::BadMELibVar)?;

  let me_align = settings_hm.get("mobile_element_alignment")
    .context(ChapulinConfigError::BadMEAlignVar)?;

  let ref_align = settings_hm.get("reference_genome_alignment")
    .context(ChapulinConfigError::BadSingleReferenceGenomeVar)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));
  let mutex_chr_assembly = Arc::new(Mutex::new(HashMap::new()));
  let mutex_me_library = Arc::new(Mutex::new(HashMap::new()));

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // TODO: write pre processing recomendations => fastq filtering, alignment

  // reference genome module
  let crg_chr_assembly = Arc::clone(&mutex_chr_assembly);

  if verbose {
    println!("\n{}\n{}{}", "Running Reference Genome module...".green(), "Reference file read: ".blue(), reference_file.cyan());
  }

  modules::reference_genome::ref_controller(
    directory,
    reference_file,
    crg_chr_assembly,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // mobile elements module
  let cme_record_collection = Arc::clone(&mutex_record_collection);

  if verbose {
    println!("\n{}\n{}{}", "Running Mobile Element module...".green(), "ME alignment file read: ".blue(), me_align.cyan());
  }

  modules::mobile_elements::me_controller(
    directory,
    me_library_file,
    me_align,
    mutex_me_library,
    cme_record_collection,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // chromosomal loci module
  let ccl_record_collection = Arc::clone(&mutex_record_collection);
  let ccl_anchor_registry = Arc::clone(&mutex_anchor_registry);

  if verbose {
    println!("\n{}\n{}{}", "Running Chromosomal Loci module...".green(), "Chromosomal alignment file read: ".blue(), ref_align.cyan());
  }

  modules::chromosomal_loci::cl_controller(
    directory.to_string(),
    ref_align.to_string(),
    ccl_record_collection,
    ccl_anchor_registry,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // peak identification module
  if verbose {
    println!("\n{}\n", "Running Peak Identification module...".green());
  }

  modules::peak_identification::pi_me_controller(
    output.to_string(),
    directory.to_string(),
    mutex_record_collection,
    mutex_anchor_registry,
    mutex_chr_assembly,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // TODO: build interphase to PostgreSQL

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
