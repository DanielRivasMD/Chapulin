
#[derive(Debug)]
pub struct MERead {
  pub mobel: String,
  pub size: i32, 
  pub flag: i32,
  pub pos: i32,
  pub cigar: String,
}

impl MERead {
  pub fn new() -> Self {
    Self {
      mobel: "".to_string(),
      size: 0,
      flag: 0,
      pos: 0,
      cigar: "".to_string(),
    }
  }

  pub fn loader(file_line: &Vec<&str>, mobile_size: i32) -> Self {
    Self {
      mobel: file_line[2].to_string(),
      size: mobile_size,
      flag: file_line[1].parse::<i32>().unwrap(),
      pos: file_line[3].parse::<i32>().unwrap(),
      cigar: file_line[5].to_string(),
    }
  }

}
