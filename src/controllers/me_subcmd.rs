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

// error handler
use crate::error::config_error::ChapulinConfigError;

////////////////////////////////////////////////////////////////////////////////////////////////////

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

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // collect settings
  let verbose = matches.is_present("verbose");

  let dry_run = matches.is_present("dry");

  let now = SystemTime::now();
  pretty_env_logger::init();

  let config = matches
    .value_of("config")
    .context(ChapulinConfigError::EmptyConfigOption)?;

  let chr_align = matches.value_of("chralign").unwrap();

  if verbose {
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

  let me_align = settings_hm
    .get("mobile_element_alignment")
    .context(ChapulinConfigError::BadMEAlignVar)?;

  let ref_align = settings_hm
    .get("reference_genome_alignment")
    .context(ChapulinConfigError::BadSingleReferenceGenomeVar)?;

  let pair_end_reference_alignment = settings_hm
    .get("pair_end_reference_alignment")
    .context(ChapulinConfigError::BadPairedReferenceGenomeVar)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  if dry_run {
    print!(
      "\n{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n",
      "Displaying settings".green(),
      "Configuration file: ".blue(), config.cyan(),
      "Directory: ".blue(), directory.cyan(),
      "Output: ".blue(), output.cyan(),
      "Error: ".blue(), errata.cyan(),
      "Reference file: ".blue(), reference_file.cyan(),
      "Mobile element library: ".blue(), me_library_file.cyan(),
      "Mobile element alignment: ".blue(), me_align.cyan(),
      "Reference alignment: ".blue(), ref_align.cyan(),
      "Paired end aligment: ".blue(), pair_end_reference_alignment.cyan(),
    );

    exit(0);
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // create output path
  let out_dir = format!("{}{}", directory, output);
  if !Path::new(&out_dir).exists() {
    create_dir_all(&out_dir)?;
  }

  // create error path
  let err_dir = format!("{}{}", directory, errata);
  if !Path::new(&err_dir).exists() {
    create_dir_all(&err_dir)?;
  }

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
    crg_chr_assembly,
  )?;

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // mobile elements module
  let cref_library = Arc::clone(&mutex_me_library);
  let cme_record_collection = Arc::clone(&mutex_record_collection);
  let cme_library = Arc::clone(&mutex_me_library);

  // TODO: commit these formating changes all together when update config
  if verbose {
    println!(
      "\n{}\n{}{}\n{}{}",
      "Running Mobile Element module...".green(),
      "Mobile element lirabry read: ".blue(),
      me_library_file.cyan(),
      "ME alignment file read: ".blue(),
      me_align.cyan()
    );
  }

  modules::fasta_read::cache_controller::cache_controller(
    subcmd,
    directory,
    me_library_file,
    cref_library,
  )?;

  modules::mobile_elements::me_controller(
    directory,
    me_align,
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
      if verbose {
        println!(
          "\n{}\n{}{}",
          "Running Chromosomal Loci module...".green(),
          "Chromosomal alignment file read: ".blue(),
          ref_align.cyan()
        );
      }

      modules::chromosomal_loci::cl_single_controller(
        directory.to_string(),
        ref_align.to_string(),
        errata.to_string(),
        ccl_record_collection,
        ccl_anchor_registry,
      )?;
    }

    "paired" => {
      if verbose {
        println!(
          "\n{}\n{}{}",
          "Running Chromosomal Loci module...".green(),
          "Chromosomal alignment file read: ".blue(),
          pair_end_reference_alignment.cyan()
        );
      }

      modules::chromosomal_loci::cl_paired_controller(
        directory.to_string(),
        pair_end_reference_alignment.to_string(),
        errata.to_string(),
        ccl_record_collection,
        ccl_anchor_registry,
      )?;
    }

    _ => (),
  }

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // peak identification module
  if verbose {
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
