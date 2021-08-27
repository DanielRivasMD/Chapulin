////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use clap::ArgMatches;
use colored::*;
use config::{
  Config,
  File,
};
use std::collections::HashMap;
use std::time::SystemTime;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  common_error::ChapulinCommonError,
  config_error::ChapulinConfigError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
use crate::modules;

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: add compatibility for known coordinates

pub fn sv_subcmd(matches: &ArgMatches) -> alias::AnyResult {
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

  let config = matches
    .value_of("config")
    .context(ChapulinConfigError::EmptyConfigOption)?;

  if verbose {
    println!(
      "\n{}\n{}{}",
      "Setting up configuration...".green(),
      "Configuration file read: ".blue(),
      config.cyan()
    );
  }

  let mut settings = Config::default();
  settings
    .merge(File::with_name(config))
    .context(ChapulinConfigError::NoConfigFile)?;

  let settings_hm = settings.try_into::<HashMap<String, String>>().context(
    ChapulinConfigError::ConfigHashMap {
      f: config.to_string(),
    },
  )?;

  let directory = settings_hm
    .get("directory")
    .context(ChapulinConfigError::BadDirectoryVar)?;

  let output = settings_hm
    .get("output")
    .context(ChapulinConfigError::BadOutput)?;

  let reference_file = settings_hm
    .get("reference")
    .context(ChapulinConfigError::BadReferenceVar)?;

  let pair_end_reference_alignment = settings_hm
    .get("pair_end_reference_alignment")
    .context(ChapulinConfigError::BadPairedReferenceGenomeVar)?;

  let expected_tlen = settings_hm
    .get("expected_tlen")
    .context(ChapulinConfigError::TODO)?
    .parse::<i32>()
    .context(ChapulinCommonError::Parsing)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let amx_sv_record = alias::arc_map();
  let amx_chr_registry = alias::arc_map();
  let amx_dir_registry = alias::arc_map();

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // reference genome module
  let camx_dir_registry = alias::arc_clone(&amx_dir_registry);

  if verbose {
    println!(
      "\n{}\n{}{}",
      "Running Reference Genome module...".green(),
      "Reference file read: ".blue(),
      reference_file.cyan()
    );
  }

  modules::fasta_read::cache_controller::cache_controller(
    subcmd,
    directory,
    reference_file,
    camx_dir_registry,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // structural variant module
  let camx_sv_record = amx_sv_record.clone();
  let camx_chr_registry = amx_chr_registry.clone();

  if verbose {
    println!(
      "\n{}\n{}{}",
      "Running Structural Variant module...".green(),
      "Alignment file read: ".blue(),
      pair_end_reference_alignment.cyan()
    );
  }

  modules::structural_variant::sv_controller(
    directory,
    expected_tlen,
    pair_end_reference_alignment,
    camx_sv_record,
    camx_chr_registry,
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
    amx_chr_registry,
    amx_dir_registry,
    amx_sv_record,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
