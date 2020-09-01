
////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::functions::{
    flag_interpretor::{
      SamFlag,
      interpretor,
    },
  }
};


////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new)]
pub struct MEAnchor {

  #[new(default)]
  pub mobel: String,

  #[new(default)]
  pub size: f64,

  #[new(default)]
  pub flag: i32,

  #[new(default)]
  pub pos: i32,

  #[new(default)]
  pub cigar: String,

  #[new(default)]
  pub orientation: String,

}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MEAnchor {

  pub fn loader(file_line: &[&str], mobile_size: f64, mobile_orientation: &str) -> Self {
    Self {
      mobel: file_line[2].to_string(),
      size: mobile_size,
      flag: file_line[1].parse::<i32>().unwrap(),
      pos: file_line[3].parse::<i32>().unwrap(),
      cigar: file_line[5].to_string(),
      orientation: (&mobile_orientation).to_string(),
    }
  }

}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl SamFlag for MEAnchor {
  fn interpretor(&self, p: usize) -> bool {
    interpretor(self.flag, p)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
