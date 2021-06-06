
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::{App};
use clap_generate::{generate, Generator};
use std::io;

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn print_completions<G: Generator>(app: &mut App) {
  generate::<G, _>(app, app.get_name().to_string(), &mut io::stdout());
}

////////////////////////////////////////////////////////////////////////////////////////////////////
