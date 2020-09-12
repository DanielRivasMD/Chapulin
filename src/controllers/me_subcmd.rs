
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
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

// crate utilities
use crate::{
  settings::{
    config::SETTINGS,
  },
};

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
  // SETTINGS
  //   .write().unwrap()
  settings
    .merge(File::with_name(config))
    .context(ChapulinConfigError::NoConfigFile)?;

  let settings_hm = settings.try_into::<HashMap<String, String>>()
    .context(ChapulinConfigError::ConfigHashMap{ f: config.to_string() })?;

  let directory = settings_hm.get("directory")
  // let directory: &'static str = SETTINGS
  //   .read().unwrap()
  //   .get("directory")
    .context(ChapulinConfigError::BadDirectoryVar)?;

  // let mutex_directory = Arc::new(settings_hm.get("directory").unwrap().to_string());

  let reference_file = settings_hm.get("reference")
  // let reference_file: &'static str = SETTINGS
  //   .read().unwrap()
  //   .get("reference")
    .context(ChapulinConfigError::BadReferenceVar)?;

  let me_library_file = settings_hm.get("mobile_element_library")
  // let me_library_file: &'static str = SETTINGS
  //   .read().unwrap()
  //   .get("mobile_element_library")
    .context(ChapulinConfigError::BadMELibVar)?;

  let me_align = settings_hm.get("mobile_element_alignment")
  // let me_align: &'static str = SETTINGS
  //   .read().unwrap()
  //   .get("mobile_element_alignment")
    .context(ChapulinConfigError::BadMEAlignVar)?;

  let cl_align = settings_hm.get("reference_genome_alignment")
  // let cl_align: &'static str = SETTINGS
  //   .read().unwrap()
  //   .get("reference_genome_alignment")
    .context(ChapulinConfigError::BadSingleReferenceGenomeVar)?;

  // let mutex_align = Arc::new(settings_hm.get("reference_genome_alignment").unwrap().to_string());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));
  let mutex_chr_assembly = Arc::new(Mutex::new(HashMap::new()));
  let mutex_me_library = Arc::new(Mutex::new(HashMap::new()));

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // TODO: write pre processing recomendations => fastq filtering, alignment

  // reference genome module
  let c_rg_chr_assembly = mutex_chr_assembly.clone();

  if verbose {
    println!("\n{}\n{}{}", "Running Reference Genome module...".green(), "Reference file read: ".blue(), reference_file.cyan());
  }

  modules::reference_genome::ref_controller(
    directory,
    reference_file,
    c_rg_chr_assembly,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // mobile elements module
  let c_me_record_collection = mutex_record_collection.clone();

  if verbose {
    println!("\n{}\n{}{}", "Running Mobile Element module...".green(), "ME alignment file read: ".blue(), me_align.cyan());
  }

  modules::mobile_elements::me_controller(
    directory,
    me_library_file,
    me_align,
    mutex_me_library,
    c_me_record_collection,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // chromosomal loci module
  let c_cl_record_collection = mutex_record_collection.clone();
  let c_cl_anchor_registry = mutex_anchor_registry.clone();

  if verbose {
    println!("\n{}\n{}{}", "Running Chromosomal Loci module...".green(), "Chromosomal alignment file read: ".blue(), cl_align.cyan());
  }

  modules::chromosomal_loci::cl_controller(
    directory,
    cl_align,
    c_cl_record_collection,
    c_cl_anchor_registry,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // peak identification module
  // let c_pi_record_collection = mutex_record_collection.clone();

  if verbose {
    println!("\n{}\n", "Running Peak Identification module...".green());
  }

  modules::peak_identification::pi_me_controller(
    directory,
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
