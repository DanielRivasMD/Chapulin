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
use std::process::exit;
use std::time::SystemTime;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::config_error::ChapulinConfigError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
use crate::modules;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cr_subcmd(matches: &ArgMatches) -> alias::AnyResult {
  let subcmd = "CR";

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

  let dry_run = matches.is_present("dry");

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

  let errata = settings_hm
    .get("error")
    .context(ChapulinConfigError::BadError)?;

  let reference_file = settings_hm
    .get("reference")
    .context(ChapulinConfigError::BadReferenceVar)?;

  let me_library_file = settings_hm
    .get("mobile_element_library")
    .context(ChapulinConfigError::BadMELibVar)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  if dry_run {
    print!(
      "\n{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n",
      "Displaying settings".green(),
      "Configuration file: ".blue(),
      config.cyan(),
      "Directory: ".blue(),
      directory.cyan(),
      "Output: ".blue(),
      output.cyan(),
      "Error: ".blue(),
      errata.cyan(),
      "Reference file: ".blue(),
      reference_file.cyan(),
      "Mobile element library: ".blue(),
      me_library_file.cyan(),
    );

    exit(0);
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let mutex_chr_assembly = alias::arc_map();
  let mutex_me_library = alias::arc_map();

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // reference genome module
  let crg_chr_assembly = alias::arc_clone(&mutex_chr_assembly);

  if verbose {
    println!(
      "\n{}\n{}{}",
      "Registering Reference Genome module...".green(),
      "Reference file read: ".blue(),
      reference_file.cyan()
    );
  }

  modules::fasta_read::cache_controller::cache_controller(
    subcmd,
    directory,
    reference_file,
    crg_chr_assembly,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // mobile elements module
  let cref_library = alias::arc_clone(&mutex_me_library);

  if verbose {
    println!(
      "\n{}\n{}{}",
      "Registering Mobile Element module...".green(),
      "Mobile element lirabry read: ".blue(),
      me_library_file.cyan()
    );
  }

  modules::fasta_read::cache_controller::cache_controller(
    subcmd,
    directory,
    me_library_file,
    cref_library,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
