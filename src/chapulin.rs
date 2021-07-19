////////////////////////////////////////////////////////////////////////////////////////////////////
// macros                                                                                         //
////////////////////////////////////////////////////////////////////////////////////////////////////

// derive new
#[macro_use]
extern crate derive_new;

// logging
#[macro_use]
extern crate log;

// lazy static
#[macro_use]
extern crate lazy_static;

// debug
#[macro_use]
extern crate icecream;

// genomic structures
#[macro_use]
extern crate genomic_structures;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
pub mod controllers;
pub mod error;
pub mod modules;
pub mod settings;
pub mod utils;

////////////////////////////////////////////////////////////////////////////////////////////////////

// use prettytable::format::LineSeparator;
// use prettytable::format::LinePosition;
// use prettytable::format::FormatBuilder;
// use prettytable::format::TableFormat;

// lazy_static! {
//     static ref COOL_SEP: LineSeparator = LineSeparator::new('\u{2256}', '\u{2256}', '\u{2256}',
// '\u{2256}');

//     pub static ref COOL_FORMAT: TableFormat = FormatBuilder::new()
//       .column_separator('\u{22EE}')
//       .borders('\u{22EE}')
//       .separator(LinePosition::Title, *COOL_SEP)
//       .separator(LinePosition::Bottom, *COOL_SEP)
//       .separator(LinePosition::Top, *COOL_SEP)
//       .padding(1, 1)
//       .build();
// }

////////////////////////////////////////////////////////////////////////////////////////////////////
