
#[derive(Debug)]
pub struct PrimaryME {
  pub read_id: String,
  pub r1proviral_flag: i64,
  pub r1mobel: String,
  pub r1proviral_pos: i64,
  pub r1proviral_cigar: String,
  pub r1read_sequence: String,
  pub r2proviral_flag: i64,
  pub r2mobel: String,
  pub r2proviral_pos: i64,
  pub r2proviral_cigar: String,
  pub r2read_sequence: String,
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

  pub fn new() -> Self {

    PrimaryME {
      read_id: "".to_string(),
      r1proviral_flag: 0,
      r1mobel: "".to_string(),
      r1proviral_pos: 0,
      r1proviral_cigar: "".to_string(),
      r1read_sequence: "".to_string(),
      r2proviral_flag: 0,
      r2mobel: "".to_string(),
      r2proviral_pos: 0,
      r2proviral_cigar: "".to_string(),
      r2read_sequence: "".to_string(),
    }
  }

  pub fn reverser(&self) -> String {
    self.read_sequence.chars().rev().collect()
  }
}