
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
