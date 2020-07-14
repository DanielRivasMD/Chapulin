
#[derive(Debug, new)]
pub struct ElementCounter {
  #[new(default)]
  pub upstream: i32,
  #[new(default)]
  pub downstream: i32,
}

impl ElementCounter {
  pub fn counter(&mut self, orientation: &String) {

    if orientation == "upstream" {
      self.upstream = self.upstream + 1;
    } else if orientation == "downstream" {
      self.downstream = self.downstream + 1;
    }
  }
}
