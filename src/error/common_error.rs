////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use colored::*;
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Error)]
pub enum ChapulinCommonError {
  #[error("\n{}: {f:?}\n", "Fail to create file".red())]
  CreateFile { f: String },

  #[error("\n{}: {f:?}\n", "Fail to read file".red())]
  ReadFile { f: String },

  #[error("\n{}: {f:?}\n", "Fail to write file".red())]
  WriteFile { f: String },

  #[error("\n{}\n", "Fail to parse".red())]
  Parsing,

  #[error("\n{}\n", "Fail to read lines".red())]
  RegistryLine,

  #[error("Error TODO")]
  TODO,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
