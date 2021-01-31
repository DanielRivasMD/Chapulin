
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use thiserror::Error;
use colored::*;

////////////////////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Error)]
pub enum ChapulinConfigError {

  #[error("\n{}: {f:?}\n", "Fail to read configuration".red())]
  ConfigHashMap {
    f: String,
  },

  #[error("\n{}\n{}{}\n\n", "Configuration file was not set:".red(), "Set configuration file with option ", "'-c --config'".cyan(), )]
  EmptyConfigOption,

  #[error("\n{}\n", "Configuration file not found".red())]
  NoConfigFile,

  #[error("\n{}\n{}{}\n", "Directory was not set properly in configuration file".red(), "Example: directory = ", "'/home/favorite_chapulin_directory/'".cyan())]
  BadDirectoryVar,

  #[error("\n{}\n{}{}\n", "Output directory was not set properly in configuration file".red(), "Example: directory = ", "'/home/chapulin_will_write_output_here/'".cyan())]
  BadOutput,

  #[error("\n{}\n{}{}\n", "Error directory was not set properly in configuration file".red(), "Example: directory = ", "'/home/chapulin_will_write_error_here/'".cyan())]
  BadError,

  #[error("\n{}\n{}{}\n", "Reference file was not set properly in configuration file".red(), "Example: reference = ", "'awesome_species_reference.fa'".cyan())]
  BadReferenceVar,

  #[error("\n{}\n{}{}\n", "Mobile element library was not set properly in configuration file".red(), "Example: mobile_element_library = ", "'cool_ME_library.txt'".cyan())]
  BadMELibVar,

  #[error("\n{}\n{}{}\n", "Mobile element alignment was not set properly in configuration file".red(), "Example: mobile_element_alignment = ", "'ME_alignment_to_awesome_species.sam'".cyan())]
  BadMEAlignVar,

  #[error("\n{}\n{}{}\n", "Single-end reference genome alignment was not set properly in configuration file".red(), "Example: reference_genome_alignment = ", "'alignment_to_awesome_species_reference_R' Note: this is a single-end alignment, therefore files shoud be: 'alignment_to_awesome_species_reference_R1.sam' & 'alignment_to_awesome_species_reference_R2.sam', where suffixes are infered".cyan())]
  BadSingleReferenceGenomeVar,

  #[error("\n{}\n{}{}\n", "Paired-end reference genome alignment was not set properly in configuration file".red(), "Example: reference_genome_alignment = ", "'alignment_to_awesome_species_reference_R' Note: this is a single-end alignment, therefore files shoud be: 'alignment_to_awesome_species_reference_R1.sam' & 'alignment_to_awesome_species_reference_R2.sam', where suffixes are infered".cyan())]
  BadPairedReferenceGenomeVar,

  #[error("Error TODO")]
  TODO,

}

////////////////////////////////////////////////////////////////////////////////////////////////////
