
#[derive(Debug)]
pub struct MobelCounter {
  pub upstream: i32,
  pub downstream: i32,
}

impl MobelCounter {
  pub fn new() -> Self {
    Self {
      upstream: 0,
      downstream: 0,
    }
  }
}