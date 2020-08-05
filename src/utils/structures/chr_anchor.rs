
////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  settings::{
    constants::BIN_SIZE,
  },
  utils::functions::{
    flag_interpretor::{
      SamFlag,
      interpretor,
    },
  }
};

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new, Default)]
pub struct ChrAnchor {
  #[new(default)]
  pub chr: String,
  #[new(default)]
  pub flag: i32,
  #[new(default)]
  pub pos: i32,
  #[new(default)]
  pub cigar: String,
  #[new(default)]
  pub mapq: i32,
  #[new(default)]
  pub tlen: i32,
}

impl ChrAnchor {

  pub fn loader(file_line: &[&str]) -> Self {
    Self {
      chr: file_line[2].to_string(),
      flag: file_line[1].parse::<i32>().unwrap(),
      pos: file_line[3].parse::<i32>().unwrap(),
      cigar: file_line[5].to_string(),
      mapq: file_line[4].parse::<i32>().unwrap(),
      tlen: file_line[8].parse::<i32>().unwrap(),
    }
  }

  pub fn binner(&self) -> i32 {
    let binned = self.pos % BIN_SIZE;
    self.pos - binned
  }

}

impl SamFlag for ChrAnchor {
  fn interpretor(&self, p: usize) -> bool {
    interpretor(self.flag, p)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
