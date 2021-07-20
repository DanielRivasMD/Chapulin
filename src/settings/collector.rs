////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use clap::ArgMatches;
use colored::*;
use std::collections::HashMap;
use std::fmt;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::config_error::ChapulinConfigError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct BoolSettings {
  pub verbose: bool,
  pub dry_run: bool,
  pub force:   bool,
}

pub fn bool_collector(matches: &ArgMatches) -> BoolSettings {
  BoolSettings {
    verbose: matches.is_present("verbose"),
    dry_run: matches.is_present("dry"),
    force:   matches.is_present("force"),
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct ParamSettings {
  pub directory: String,
  pub output: String,
  pub errata: String,
  pub reference_file: String,
  pub me_library_file: String,
  pub me_align: String,
  pub ref_align: String,
  pub pair_end_reference_alignment: String,
}

pub fn str_collector(sett_hm: HashMap<String, String>) -> anyResult<ParamSettings> {
  Ok(ParamSettings {
    directory: sett_hm
      .get("directory")
      .context(ChapulinConfigError::BadDirectoryVar)?
      .to_string(),

    output: sett_hm
      .get("output")
      .context(ChapulinConfigError::BadOutput)?
      .to_string(),

    errata: sett_hm
      .get("error")
      .context(ChapulinConfigError::BadError)?
      .to_string(),

    reference_file: sett_hm
      .get("reference")
      .context(ChapulinConfigError::BadReferenceVar)?
      .to_string(),

    me_library_file: sett_hm
      .get("mobile_element_library")
      .context(ChapulinConfigError::BadMELibVar)?
      .to_string(),

    me_align: sett_hm
      .get("mobile_element_alignment")
      .context(ChapulinConfigError::BadMEAlignVar)?
      .to_string(),

    ref_align: sett_hm
      .get("reference_genome_alignment")
      .context(ChapulinConfigError::BadSingleReferenceGenomeVar)?
      .to_string(),

    pair_end_reference_alignment: sett_hm
      .get("pair_end_reference_alignment")
      .context(ChapulinConfigError::BadPairedReferenceGenomeVar)?
      .to_string(),
  })
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// display trait implementation. colorized
impl fmt::Display for ParamSettings {
  fn fmt(
    &self,
    f: &mut fmt::Formatter,
  ) -> fmt::Result {
    writeln!(
      f,
      "\n{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}\n{:<30}{}",
      "Displaying settings".green(),
      "Directory: ".blue(), self.directory.cyan(),
      "Output: ".blue(), self.output.cyan(),
      "Error: ".blue(), self.errata.cyan(),
      "Reference file: ".blue(), self.reference_file.cyan(),
      "Mobile element library: ".blue(), self.me_library_file.cyan(),
      "Mobile element alignment: ".blue(), self.me_align.cyan(),
      "Reference alignment: ".blue(), self.ref_align.cyan(),
      "Paired end aligment: ".blue(), self.pair_end_reference_alignment.cyan(),
    )
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
