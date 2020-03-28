
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

// to load onto => hashmap for reads primary aligned to mobile elements
#[derive(Debug)]
pub struct PrimaryME {
  // pub read_id: String,
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

// to load onto => hashmap for reads secondary aligned to mobile elements
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
      // read_id: "".to_string(),
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

  // TODO: write reverse trait
  
  pub fn reverser(&self) -> String {

    self.r1read_sequence.chars().rev().collect()
  }

//  TODO: add breakpoint determination as trait
}
