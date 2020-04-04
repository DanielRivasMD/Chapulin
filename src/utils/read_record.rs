
use std::collections::HashMap;

pub fn hashmap_init<T>() -> HashMap < String, T > {
  let out_hashmap: HashMap < String, T > = HashMap::new();
  return out_hashmap
}

// TODO: finish mobile element library struct
// to load onto => hashmap for mobile elements library entries
#[derive(Debug)]
pub struct MElibrary {
  pub me_seq: String,
  pub me_size: i32,
  pub annotations_erv: ERVannoations,
  // potentially expandable to other types of mobile elements
}

// use to indicate which LTR sequence to use
#[derive(Debug)]
pub struct ERVannoations {
  pub ltr5: bool,
  pub ltr3: bool,
}

impl MElibrary {
  pub fn new() -> Self {
    Self {
      me_seq: "".to_string(),
      me_size: 0,
      annotations_erv: ERVannoations {
        ltr5: false,
        ltr3: false
      }
    }
  }
}

#[derive(Debug)]
pub struct ReadSequence {
  pub sequence: String,
  pub test_seq: String,
  pub mobel: String,
  pub pv_flag: i32,
  pub pv_pos: i32,
  pub pv_cigar: String,
  pub chr: String,
  pub cl_flag: i32,
  pub cl_pos: i32,
  pub cl_cigar: String,
  pub cl_mapq: String,
}

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
      cl_mapq: "".to_string(),
    }
  }
}

impl ReadSequence {
  pub fn sequence_reverser(&self) -> String {

    self.sequence.chars()
    .map(|x| match x {
        '!' => '?',
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => x
    }).rev().collect()
  }

  //  TODO: add breakpoint determination as trait
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

#[derive(Debug)]
pub struct CIGAR {
  pub lclip: i32,
  pub align: Vec<i32>,
  pub rclip: i32,
  pub ins: Vec<i32>,
  pub del: Vec<i32>,
}

impl CIGAR {
  pub fn new() -> Self {
    Self {
      lclip: 0,
      align: Vec::new(),
      rclip: 0,
      ins: Vec::new(),
      del: Vec::new(),
    }
  }
}
