
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
