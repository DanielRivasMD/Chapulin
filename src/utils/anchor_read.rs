
#[derive(Debug)]
pub struct AnchorRead {
  pub chr: String,
  pub flag: i32,
  pub pos: i32,
  pub cigar: String,
  pub mapq: i32,
}

impl AnchorRead {
  pub fn new() -> Self {
    Self {
      chr: "".to_string(),
      flag: 0,
      pos: 0,
      cigar: "".to_string(),
      mapq: 0
    }
  }

  pub fn loader(file_line: &Vec<&str>) -> Self {
    Self {
      chr: file_line[2].to_string(),
      flag: file_line[1].parse().unwrap(),
      pos: file_line[3].parse().unwrap(),
      cigar: file_line[5].to_string(),
      mapq: file_line[4].parse().unwrap(),
    }
  }

}
