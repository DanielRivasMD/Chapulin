
// to load onto => hashmap for reads aligned to mobile elements
#[derive(Debug)]
pub struct ReadRecord {
  pub read1: ReadSequence,
  pub read2: ReadSequence,
  pub debug_seq: String,
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

// annotate primary (index 0) & secondary aligned reads
#[derive(Debug)]
pub struct ReadSequence {
  pub sequence: String,
  pub test_seq: String,
  pub me_read: Vec<MERead>,
  pub chr_read: Vec<AnchorRead>,
}

impl ReadSequence {
  pub fn new() -> Self {
    Self {
      sequence: "".to_string(),
      test_seq: "".to_string(),
      me_read: vec![MERead::new()],
      chr_read: vec![AnchorRead::new()],
    }
  }
}

impl ReadSequence {

  // reverse complement sequence
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

#[derive(Debug)]
pub struct MERead {
  pub mobel: String,
  pub flag: i32,
  pub pos: i32,
  pub cigar: String,
}

impl MERead {
  pub fn new() -> Self {
    Self {
      mobel: "".to_string(),
      flag: 0,
      pos: 0,
      cigar: "".to_string(),
    }
  }
}

#[derive(Debug)]
pub struct AnchorRead {
  pub chr: String,
  pub flag: i32,
  pub pos: i32,
  pub cigar: String,
  pub mapq: String,
}

impl AnchorRead {
  pub fn new() -> Self {
    Self {
      chr: "".to_string(),
      flag: 0,
      pos: 0,
      cigar: "".to_string(),
      mapq: "".to_string()
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
