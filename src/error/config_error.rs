
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use thiserror::Error;
use colored::*;

////////////////////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Error)]
pub enum ChapulinConfigError {
  #[error("\n{}\n", "Fail to reaad configuration".red())]
  ConfigHashMap,
  #[error("\n\t{}\n{}{}", "No configuration file not was set:".red(), "Set a configuration file with option ", "'-c --config'".cyan(), )]
  EmptyConfigOption,
  #[error("\n\t{}", "Configuration file not found".red())]
  NoConfigFile,
  #[error("\n\t{}\n{}{}", "Directory was not set properly in configuration file".red(), "Example: directory = ", "'/home/favorite_chapulin_directory/'".cyan())]
  BadDirectoryVar,
  #[error("\n\t{}\n{}{}", "Reference file was not set properly in configuration file".red(), "Example: reference = ", "'awesome_species_reference.fa'".cyan())]
  BadReferenceVar,
  #[error("\n\t{}\n{}{}", "Mobile element library was not set properly in configuration file".red(), "Example: mobile_element_library = ", "'cool_ME_library.txt'".cyan())]
  BadMELibVar,
  #[error("\n\t{}\n{}{}", "Mobile element alignment was not set properly in configuration file".red(), "Example: mobile_element_alignment = ", "'ME_alignment_to_awesome_species.sam'".cyan())]
  BadMEAlignVar,
  #[error("\n\t{}\n{}{}", "Reference genome alignment was not set properly in configuration file".red(), "Example: reference_genome_alignment = ", "'alignment_to_awesome_species_reference_R' Note: this is a single-end alignment, therefore files shoud be: 'alignment_to_awesome_species_reference_R1.sam' & 'alignment_to_awesome_species_reference_R2.sam', where suffixes are infered".cyan())]
  BadReferenceGenomeVar,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
