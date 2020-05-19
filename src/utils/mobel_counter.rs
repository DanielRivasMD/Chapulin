
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

impl MobelCounter {
  pub fn counter(&mut self, orientation: &String) {

    if orientation == "upstream" {
      self.upstream = self.upstream + 1;
    } else if orientation == "downstream" {
      self.downstream = self.downstream + 1;
    }
  }
}
