
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::time::{SystemTime};
use clap::{ArgMatches};
use clap_generate::{generators::{*}};
use anyhow::{Context};
use anyhow::Result as anyResult;
use colored::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::cli::{
    help::cli_chapulin,
    completion::print_completions,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  config_error::ChapulinConfigError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn ac_subcmd(
  matches: &ArgMatches
) -> anyResult<()> {

  let _subcmd = "AC";

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

  let shell = matches
    .value_of("shell")
    .context(ChapulinConfigError::TODO)?;

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  if verbose {
    println!("\n{}\n", "Generating autocompletions...".green());
  }

  let mut app = cli_chapulin();

  match shell {
    "bash" => print_completions::<Bash>(&mut app),
    "elvish" => print_completions::<Elvish>(&mut app),
    "fish" => print_completions::<Fish>(&mut app),
    "powershell" => print_completions::<PowerShell>(&mut app),
    "zsh" => print_completions::<Zsh>(&mut app),
    _ => panic!("Unknown generator"),
  }

  info!("{:?}", now.elapsed().unwrap());

  ////////////////////////////////////////////////////////////////////////////////////////////////////

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
