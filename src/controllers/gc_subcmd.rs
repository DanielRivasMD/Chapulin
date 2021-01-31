
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::time::{SystemTime};
use clap::{ArgMatches};
use std::fs::{File};
use std::io::{Write};
use std::path::{Path};
use anyhow::{Context};
use anyhow::Result as anyResult;
use colored::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  config_error::ChapulinConfigError,
  common_error::ChapulinCommonError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn gc_subcmd(
  matches: &ArgMatches
) -> anyResult<()> {

  let _subcmd = "GC";

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

  let force = matches.is_present("force");

  let now = SystemTime::now();
  pretty_env_logger::init();

  let mut config = "chapulin_config".to_string();
  if let Some(cname) = matches.value_of("CONFIG") {
    config = cname.to_string();
  }

  if verbose {
    println!("\n{}\n{}{}", "Writting configuration...".green(), "Configuration file read: ".blue(), config.cyan());
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  let fl_conf = format!("{}.toml", config);

  if Path::new(&fl_conf).exists() {
    info!("Please observe that path exists: {}", config);
    if force {
      info!("Excesive force. Overwritting!", );
      write_conf(fl_conf)?;
    } else {
      info!("If you wish to overwrite, run command with option --force", );
    }
  } else {
    write_conf(fl_conf)?;
  }

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////


fn write_conf(
  fl_write: String,
) -> anyResult<()> {
  info!("Writting now!", );

  let mut fl = File::create(&fl_write).context(ChapulinCommonError::CreateFile{ f: fl_write })?;

  // ("directory", "")
  // ("output", "")
  // ("error", "")
  // ("reference", "")
  // ("mobile_element_library", "")
  // ("mobile_element_alignment", "")
  // ("reference_genome_alignment", "")
  // ("pair_end_reference_alignment", "")

  let to_write = format!("{} = {}\n", "key", "value");
  fl.write_all(to_write.as_bytes()).context(ChapulinCommonError::WriteFile{ f: to_write })?;

  Ok(())

}

////////////////////////////////////////////////////////////////////////////////////////////////////


