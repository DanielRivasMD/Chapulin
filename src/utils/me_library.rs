
// to load onto => hashmap for mobile elements library entries
#[derive(Debug)]
pub struct MElibrary {
  pub me_seq: String,
  pub me_size: i32,
  pub annotations_erv: ERVannoations,
  // potentially expandable to other types of mobile elements
}

// use to indicate which LTR sequence to use
#[derive(Debug)]
pub struct ERVannoations {
  pub ltr5: bool,
  pub ltr3: bool,
}

impl MElibrary {
  pub fn new() -> Self {
    Self {
      me_seq: "".to_string(),
      me_size: 0,
      annotations_erv: ERVannoations {
        ltr5: false,
        ltr3: false
      }
    }
  }
}
