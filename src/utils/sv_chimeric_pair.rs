
// crate utilities
use crate::utils::{
  sv_chimeric_read::SVChimericRead,
  sv_type::SVType,
};

// to load onto => hashmap for reads aligned to mobile elements
#[derive(Debug, new)]
pub struct SVChimericPair {
  #[new(default)]
  pub read1: SVChimericRead,
  #[new(default)]
  pub read2: SVChimericRead,
  pub svtag: SVType,
}
