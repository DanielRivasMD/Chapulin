
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use thiserror::Error;
use colored::*;

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: perhaps use with context for common errors
#[derive(Debug, Error)]
pub enum ChapulinCommonError {

  #[error("\n{}: {f:?}\n", "Cannot create file".red())]
  CreateFile {
    f: String,
  },

  #[error("\n{}: {f:?}\n", "Cannot read file".red())]
  ReadFile {
    f: String,
  },

  #[error("\n{}: {f:?}\n", "Cannot write file".red())]
  WriteFile {
    f: String,
  },
  #[error("\n{}\n", "Fail to read lines".red())]
  Parsing,
  #[error("\n{}\n", "Fail to read lines".red())]
  RegistryLine,
  #[error("\n\t{}\n{}{}", "No configuration file not was set:".red(), "Set a configuration file with option ", "'-c --config'".cyan(), )]
  TODO,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
