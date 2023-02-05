// TODO: add one chromosome from reference NA to example
////////////////////////////////////////////////////////////////////////////////////////////////////

// chapulin wrapper
use chapulin::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::{Parser, Subcommand};

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::cli::help::{Cli, Commands};

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: update tool information

/// the general idea is to create a modulerize, fast & reliable tool for mobile
/// element identification in re sequence projects
///
/// hold all configuration variables in one file
/// read bam files, one from mobile element alignment & two from chromosomal
/// reference alingment, once from disk load all neccesary data into memory
/// (hashmap) for efficiency. to optimize this process, use two methods:
///   1) prefilter read to load, therefore minimizing size of hashmap to hold
///   2) load all neccesary data into each struct record and use traits to hold
/// virtual information
///
/// collect both, mobile element & chromosomal reference, versions of insert
/// pairs filter according to quality criteria
/// perform peak detection & calculate false discovery rate
/// label chimeric reads for massive break point reconstructions
/// generate stats at every step
/// create a safe escape in case of memory failures
/// create unit tests

////////////////////////////////////////////////////////////////////////////////////////////////////

fn main() -> alias::AnyResult {

  let params = Cli::parse();

  match &params.command.unwrap() {

    Commands::ME { chralign, debug } => {
      println!("ME");

      // controllers::me_subcmd::me_subcmd(params)?;
    }

  }

  // let matches = cli_chapulin().get_matches();

  // TODO: add single-end reference read support by interpreting CIGAR
  //  TODO: replace if statements for match / switch

  // // FIX: better interphase for commands
  // #[derive(Debug)]
  // struct SubCmds(String, String);

  // let subcms_vec = vec![
  //   SubCmds("AC".to_string(), "AC".to_string()),
  //   SubCmds("CR".to_string(), "CR".to_string()),
  //   SubCmds("GC".to_string(), "GC".to_string()),
  //   SubCmds("ME".to_string(), "ME".to_string()),
  //   SubCmds("SV".to_string(), "SV".to_string()),
  // ];

  // // AC controller
  // if let Some(matches) = matches.subcommand_matches("AC") {
  //   controllers::ac_subcmd::ac_subcmd(matches)?;
  // }

  // CC controller
  if let Some(matches) = matches.subcommand_matches("CR") {
    controllers::cr_subcmd::cr_subcmd(matches)?;
  }

  // GC controller
  if let Some(matches) = matches.subcommand_matches("GC") {
    controllers::gc_subcmd::gc_subcmd(matches)?;
  }

  // ME controller
  if let Some(matches) = matches.subcommand_matches("ME") {
    controllers::me_subcmd::me_subcmd(matches)?;
  }

  // // SV controller
  // if let Some(matches) = matches.subcommand_matches("SV") {
  //   controllers::sv_subcmd::sv_subcmd(matches)?;
  // }

  // T controller
  if let Some(matches) = matches.subcommand_matches("T") {
    controllers::t_subcmd::t_subcmd(matches)?;
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handling: what went wrong, why it went wrong & how to fix it
