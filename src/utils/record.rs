
use std::collections::HashMap;

pub fn hashmap_init() -> HashMap < String, ReadRecord > {
  let out_hashmap: HashMap < String, ReadRecord > = HashMap::new();
  return out_hashmap
}

// TODO: finish mobile element library struct
// to load onto => hashmap for mobile elements library entries
#[derive(Debug)]
pub struct MElibrary {
  pub me_id: String,
  pub me_sequence: String,
  pub annotations_erv: ERVannoations,
  // potentially expandable to other types of mobile elements
}

// use to indicate which LTR sequence to use
#[derive(Debug)]
pub struct ERVannoations {
  pub ltr5: bool,
  pub ltr3: bool,
}

#[derive(Debug)]
pub struct ReadSequence {
  pub sequence: String,
  pub test_seq: String,
  pub mobel: String,
  pub pv_flag: i64,
  pub pv_pos: i64,
  pub pv_cigar: String,
  pub chr: String,
  pub cl_flag: i64,
  pub cl_pos: i64,
  pub cl_cigar: String,
  // TODO: expand Record to include chromosomal loci information
}

// TODO: write an automatic loader implementation to keep it clean
impl ReadSequence {
  pub fn new() -> Self {
    Self {
      sequence: "".to_string(),
      test_seq: "".to_string(),
      mobel: "".to_string(),
      pv_flag: 0,
      pv_pos: 0,
      pv_cigar: "".to_string(),
      chr: "".to_string(),
      cl_flag: 0,
      cl_pos: 0,
      cl_cigar: "".to_string(),
    }

  }
}

// to load onto => hashmap for reads primary aligned to mobile elements
#[derive(Debug)]
pub struct ReadRecord {
  pub read1: ReadSequence,
  pub read2: ReadSequence,
  pub debug_seq: String,
  // TODO: think about a convenient way to flag insert pairs
  // pub anchor: bool,
}

impl ReadRecord {
  pub fn new() -> Self {
    Self {
      read1: ReadSequence::new(),
      read2: ReadSequence::new(),
      debug_seq: "".to_string(),
    }
  }
}

// // to load onto => hashmap for reads secondary aligned to mobile elements
// #[derive(Debug)]
// pub struct SecondaryME {
//   pub read_id: String,
//   pub proviral_flag: i64,
//   pub mobel: String,
//   pub proviral_pos: i64,
//   pub proviral_cigar: String,
// }

// impl Record {
//
//   // TODO: write reverse trait
//
//   pub fn reverser(&self) -> String {
//
//     self.r1read_sequence.chars().rev().collect()
//   }
//
// //  TODO: add breakpoint determination as trait
// }
