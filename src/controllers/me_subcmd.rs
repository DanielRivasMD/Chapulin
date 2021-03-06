////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use clap::ArgMatches;
use colored::*;
use config::{
  Config,
  File,
};
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::path::Path;
use std::process::exit;
use std::sync::{
  Arc,
  Mutex,
};
use std::time::SystemTime;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
use crate::modules;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::settings::collector::{
  bool_collector,
  str_collector,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::config_error::ChapulinConfigError;

////////////////////////////////////////////////////////////////////////////////////////////////////

/// Control mobile element protocol.
///   - Load settings.
///   - Read and write cache.
///   - [Load mobile element alignment](modules::mobile_elements::me_aligned::me_identificator).
///   - [Load chromosomal alignment](modules::chromosomal_loci::cl_aligned::cl_mapper).
pub fn me_subcmd(matches: &ArgMatches) -> anyResult<()> {
  let subcmd = "ME";

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

  let now = SystemTime::now();
  pretty_env_logger::init();

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // collect settings
  let bool_sett = bool_collector(matches);

  // TODO: probably expand configuration?

  let config = matches
    .value_of("config")
    .context(ChapulinConfigError::EmptyConfigOption)?;

  // TODO: relocate into paramsettings
  let chr_align = matches.value_of("chralign").unwrap();

  if bool_sett.verbose {
    println!(
      "\n{}\n{}{}",
      "Setting up configuration...".green(),
      "Configuration file read: ".blue(),
      config.cyan()
    );
  }

  // TODO: parse RepeatModeler fasta names

  let mut settings = Config::default();
  settings
    .merge(File::with_name(config))
    .context(ChapulinConfigError::NoConfigFile)?;

  let settings_hm = settings.try_into::<HashMap<String, String>>().context(
    ChapulinConfigError::ConfigHashMap {
      f: config.to_string(),
    },
  )?;

  let string_sett = str_collector(settings_hm)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // print dry run
  if bool_sett.dry_run {
    println!("{}", string_sett);
    exit(0);
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // create output path
  let out_dir = format!("{}{}", string_sett.directory, string_sett.output);
  if !Path::new(&out_dir).exists() {
    create_dir_all(&out_dir)?;
  }

  // create error path
  let err_dir = format!("{}{}", string_sett.directory, string_sett.errata);
  if !Path::new(&err_dir).exists() {
    create_dir_all(&err_dir)?;
  }

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));
  let mutex_chr_assembly = Arc::new(Mutex::new(HashMap::new()));
  let mutex_me_library = Arc::new(Mutex::new(HashMap::new()));

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // TODO: write pre processing recomendations => fastq filtering, alignment

  // reference genome module
  let crg_chr_assembly = Arc::clone(&mutex_chr_assembly);

  if bool_sett.verbose {
    println!(
      "\n{}\n{}{}",
      "Running Reference Genome module...".green(),
      "Reference file read: ".blue(),
      string_sett.reference_file.cyan()
    );
  }

  modules::fasta_read::cache_controller::cache_controller(
    subcmd,
    &string_sett.directory,
    &string_sett.reference_file,
    crg_chr_assembly,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // mobile elements module
  let cref_library = Arc::clone(&mutex_me_library);
  let cme_record_collection = Arc::clone(&mutex_record_collection);
  let cme_library = Arc::clone(&mutex_me_library);

  // TODO: commit these formating changes all together when update config
  if bool_sett.verbose {
    println!(
      "\n{}\n{}{}\n{}{}",
      "Running Mobile Element module...".green(),
      "Mobile element lirabry read: ".blue(),
      string_sett.me_library_file.cyan(),
      "ME alignment file read: ".blue(),
      string_sett.me_align.cyan()
    );
  }

  modules::fasta_read::cache_controller::cache_controller(
    subcmd,
    &string_sett.directory,
    &string_sett.me_library_file,
    cref_library,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  modules::mobile_elements::me_controller(
    &string_sett.directory,
    &string_sett.me_align,
    cme_library,
    cme_record_collection,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // chromosomal loci module
  let ccl_record_collection = Arc::clone(&mutex_record_collection);
  let ccl_anchor_registry = Arc::clone(&mutex_anchor_registry);

  match chr_align {
    "single" => {
      if bool_sett.verbose {
        println!(
          "\n{}\n{}{}",
          "Running Chromosomal Loci module...".green(),
          "Chromosomal alignment file read: ".blue(),
          string_sett.ref_align.cyan()
        );
      }

      modules::chromosomal_loci::cl_single_controller(
        string_sett.directory.to_string(),
        string_sett.ref_align.to_string(),
        string_sett.errata,
        ccl_record_collection,
        ccl_anchor_registry,
      )?;
    }

    "paired" => {
      if bool_sett.verbose {
        println!(
          "\n{}\n{}{}",
          "Running Chromosomal Loci module...".green(),
          "Chromosomal alignment file read: ".blue(),
          string_sett.pair_end_reference_alignment.cyan()
        );
      }

      modules::chromosomal_loci::cl_paired_controller(
        string_sett.directory.to_string(),
        string_sett.pair_end_reference_alignment.to_string(),
        string_sett.errata,
        ccl_record_collection,
        ccl_anchor_registry,
      )?;
    }

    _ => (),
  }

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // peak identification module
  if bool_sett.verbose {
    println!("\n{}\n", "Running Peak Identification module...".green());
  }

  modules::peak_identification::pi_me_controller(
    out_dir,
    err_dir,
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
