////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use clap::ArgMatches;
use colored::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::time::SystemTime;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::settings::collector::bool_collector;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  common_error::ChapulinCommonError,
  config_error::ChapulinConfigError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn gc_subcmd(matches: &ArgMatches) -> anyResult<()> {
  let _subcmd = "GC";

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

  let now = SystemTime::now();
  pretty_env_logger::init();
  let bool_sett = bool_collector(matches);

  let mut config = "chapulin_config".to_string();
  if let Some(cname) = matches.value_of("config") {
    config = cname.to_string();
  }

  if bool_sett.verbose {
    println!(
      "\n{}\n{}{}",
      "Writting configuration...".green(),
      "Configuration file read: ".blue(),
      config.cyan()
    );
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  if bool_sett.dry_run {
    print!(
      "\n{}\n{:<30}{}\n",
      "Displaying settings".green(),
      "Configuration file: ".blue(), config.cyan(),
      // "Directory: ".blue(), directory.cyan(),
      // "Output: ".blue(), output.cyan(),
      // "Error: ".blue(), errata.cyan(),
      // "Reference file: ".blue(), reference_file.cyan(),
      // "Mobile element library: ".blue(), me_library_file.cyan(),
      // "Mobile element alignment: ".blue(), me_align.cyan(),
      // "Reference alignment: ".blue(), ref_align.cyan(),
      // "Paired end aligment: ".blue(), pair_end_reference_alignment.cyan(),
    );

    exit(0);
  }

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  // TODO: toml or not toml
  let fl_conf = format!("{}.toml", config);

  if Path::new(&fl_conf).exists() {
    print!(
      "\n{}{}\n",
      "Please observe that path exists: ".blue(),
      config.cyan()
    );
    if bool_sett.force {
      print!("\n{}\n", "Overwritting!".blue());
      write_conf(fl_conf)?;
    } else {
      print!(
        "\n{}{}\n",
        "If you wish to overwrite, run command with option ".blue(),
        "--force".cyan()
      );
    }
  } else {
    write_conf(fl_conf)?;
  }

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn write_conf(fl_write: String) -> anyResult<()> {
  info!("Writting...",);

  let mut fl =
    File::create(&fl_write).context(ChapulinCommonError::CreateFile {
      f: fl_write,
    })?;

  // ("directory", "")
  // ("output", "")
  // ("error", "")
  // ("reference", "")
  // ("mobile_element_library", "")
  // ("mobile_element_alignment", "")
  // ("reference_genome_alignment", "")
  // ("pair_end_reference_alignment", "")

  let to_write = format!("{} = {}\n", "key", "value");
  fl.write_all(to_write.as_bytes()).context(
    ChapulinCommonError::WriteFile {
      f: to_write
    },
  )?;

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
