
// crate utilities
use crate::utils::{
  erv_annotations::ERVAnnoations,
};

// to load onto => hashmap for mobile elements library entries
#[derive(Debug)]
pub struct MElibrary {
  pub me_seq: String,
  pub me_size: i32,
  pub annotations_erv: ERVAnnoations,
  // potentially expandable to other types of mobile elements
}

impl MElibrary {
  pub fn new() -> Self {
    Self {
      me_seq: "".to_string(),
      me_size: 0,
      annotations_erv: ERVAnnoations {
        ltr5: false,
        ltr3: false
      }
    }
  }
}
