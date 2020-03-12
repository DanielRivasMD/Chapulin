
// use crate::traits::*;

			// read_id = $1;
			// proviral_flag = $2;
			// erv = $3;
			// proviral_pos = $4;
			// proviral_cigar = $6;
			// to_reverse = seq;

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

// impl ReadSequence for PrimaryME {}
//
//
// // trait ReadSequence {
// pub fn reverser(to_rev_seq: &String) -> String {
//     to_rev_seq.chars().rev().collect()
//   }
// // }
