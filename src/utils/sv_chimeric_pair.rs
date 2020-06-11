
// standard libraries

// crate utilities
use crate::utils::{
  sv_chimeric_read::SVChimericRead,
  sv_type::SVType,
};

// to load onto => hashmap for reads aligned to mobile elements
#[derive(Debug)]
pub struct SVChimericPair {
  pub read1: SVChimericRead,
  pub read2: SVChimericRead,
  pub svtag: SVType,
}
// TODO: add non-cigar anchor identification

impl SVChimericPair {
  pub fn new() -> Self {
    Self {
      read1: SVChimericRead::new(),
      read2: SVChimericRead::new(),
      svtag: SVType::None,
    }
  }
}
