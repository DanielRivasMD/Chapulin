
// standard libraries
#[macro_use]
extern crate lazy_static;
use regex::Regex;

// modules
pub mod utils;
pub mod modules;

// define regex static
lazy_static! {

  pub static ref REX_VEC: Vec<Regex> = vec![
    Regex::new(
      r"(?P<clip>\d+[HS])"
    ).unwrap(),

    Regex::new(
      r"(?P<clip>\d+M)"
    ).unwrap(),

    Regex::new(
      r"(?P<clip>\d+D)"
    ).unwrap(),

    Regex::new(
      r"(?P<clip>\d+I)"
    ).unwrap(),
  ];
}


pub fn with_love() {

  println!("from lib rs with love!");
}