
#[derive(Debug)]
pub struct PrimaryME {
  pub read_id: String,
  pub proviral_flag: i64,
  pub mobel: String,
  pub proviral_pos: i64,
  pub proviral_cigar: String,
  pub read_sequence: String,
}

#[derive(Debug)]
pub struct SecondaryME {
  pub read_id: String,
  pub proviral_flag: i64,
  pub mobel: String,
  pub proviral_pos: i64,
  pub proviral_cigar: String,
}

impl PrimaryME {
  pub fn reverser(&self) -> String {
    self.read_sequence.chars().rev().collect()
  }
}
