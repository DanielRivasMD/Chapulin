
#[derive(Debug, new)]
pub struct CIGAR {
  #[new(default)]
  pub lclip: i32,
  #[new(default)]
  pub align: Vec<i32>,
  #[new(default)]
  pub rclip: i32,
  #[new(default)]
  pub ins: Vec<i32>,
  #[new(default)]
  pub del: Vec<i32>,
}

impl CIGAR {

  pub fn loader(to_interpret: &String) -> Self {

    // create new CIGAR with empty values
    let mut this_cigar = CIGAR::new();

    if to_interpret == "*" {
      this_cigar.align.push(0);
    } else {

      // iterate to find annotations
      let mut char_vec = vec![];
      for i in to_interpret.char_indices() {

        match i.1 {
          'H' | 'S' | 'M' | 'D' | 'I' => {
            char_vec.push(i.0);
          }
          _ => (),
        }
      }

      let mut j = 0;

      for i in char_vec.iter() {

        match &to_interpret[*i..*i + 1] {
          "H" | "S" => {
            if this_cigar.align.iter().sum::<i32>() == 0 {
              this_cigar.lclip = (&to_interpret[j..*i]).parse::<i32>().unwrap();
            } else {
              this_cigar.rclip = (&to_interpret[j..*i]).parse::<i32>().unwrap();
            }
          },
          "M" => {
            this_cigar.align.push((&to_interpret[j..*i]).parse::<i32>().unwrap());
          },
          "I" => {
            this_cigar.ins.push((&to_interpret[j..*i]).parse::<i32>().unwrap());
          },
          "D" => {
            this_cigar.del.push((&to_interpret[j..*i]).parse::<i32>().unwrap());
          },
          _ => {},
        }
        j = i + 1;
      };
    }
    return this_cigar
  }

  // tupple: first => adjusted position; second => total aligned length
  pub fn adjuster(&self, position: i32) -> (i32, i32) {
    let align_sum: i32 = self.align.iter().sum();
    let ins_sum: i32 = self.ins.iter().sum();
    let del_sum: i32 = self.del.iter().sum();
    (self.lclip + position, align_sum + ins_sum + del_sum)
  }

  pub fn left_boundry(&self, position: i32) -> i32 {
    position - self.lclip
  }

  pub fn right_boundry(&self, position: i32) -> i32 {
    let lpos = self.left_boundry(position);
    lpos + ( self.lclip + self.adjuster(position).1 + self.rclip )
  }
}
