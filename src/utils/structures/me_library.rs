
////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::structures::{
    erv_annotations::ERVAnnoations,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// to load onto => hashmap for mobile elements library entries
#[derive(Debug, new, Default)]
pub struct MElibrary {
  #[new(default)]
  pub me_seq: String,
  #[new(default)]
  pub me_size: i32,
  #[new(default)]
  pub annotations_erv: ERVAnnoations,
  // potentially expandable to other types of mobile elements
}

////////////////////////////////////////////////////////////////////////////////////////////////////