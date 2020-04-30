
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
