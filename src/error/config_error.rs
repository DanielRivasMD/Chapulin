
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use thiserror::Error;


////////////////////////////////////////////////////////////////////////////////////////////////////


#[derive(Debug, Error)]
pub enum ChapulinConfigError {
  #[error("\n\nFail to reaad configuration\n\n")]
  ConfigHashMap,
  #[error("\n\nNo configuration file was set:\nSet a configuration file with option '-c --config'\n\n")]
  EmptyConfigOption,
  #[error("\n\nConfiguration file not found\n\n")]
  NoConfigFile,
  #[error("\n\nDirectory was not set properly in configuration file\n\nExample: directory = \"/home/favorite_chapulin_directory/\"\n\n")]
  BadDirectoryVar,
  #[error("\n\nReference file was not set properly in configuration file\n\nExample: reference = \"awesome_species_reference.fa\"\n\n")]
  BadReferenceVar,
  #[error("\n\nMobile element library was not set properly in configuration file\n\nExample: mobile_element_library = \"cool_ME_library.txt\"\n\n")]
  BadMELibVar,
  #[error("\n\nMobile element alignment was not set properly in configuration file\n\nExample: mobile_element_alignment = \"ME_alignment_to_awesome_species.sam\"\n\n")]
  BadMEAlignVar,
  #[error("\n\nReference genome alignment was not set properly in configuration file\n\nExample: reference_genome_alignment = \"alignment_to_awesome_species_reference_R\"\n\nNote: this is a single-end alignment, therefore files shoud be: \n\t\"alignment_to_awesome_species_reference_R1.sam\" & \"alignment_to_awesome_species_reference_R2.sam\",\nwhere suffixes are infered\n\n")]
  BadReferenceGenomeVar,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
