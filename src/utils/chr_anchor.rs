
// crate utilities
use crate::{
  settings::{
    constants::BIN_SIZE,
  },
};

#[derive(Debug)]
pub struct ChrAnchor {
  pub chr: String,
  pub flag: i32,
  pub pos: i32,
  pub cigar: String,
  pub mapq: i32,
  pub tlen: i32,
}

impl ChrAnchor {
  pub fn new() -> Self {
    Self {
      chr: "".to_string(),
      flag: 0,
      pos: 0,
      cigar: "".to_string(),
      mapq: 0,
      tlen: 0
    }
  }

  pub fn loader(file_line: &Vec<&str>) -> Self {
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
