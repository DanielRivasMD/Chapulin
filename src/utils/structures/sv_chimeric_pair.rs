
////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::{
    functions::{
      identificator::identificator, 
    }, 
    structures::{
      sv_chimeric_read::SVChimericRead,
      sv_type::SVType,
    }, 
  }, 
};


////////////////////////////////////////////////////////////////////////////////////////////////////

// to load onto => hashmap for reads aligned to mobile elements
#[derive(Debug, new)]
pub struct SVChimericPair {
  #[new(default)]
  pub read1: SVChimericRead,
  #[new(default)]
  pub read2: SVChimericRead,
  pub svtag: SVType,
impl SVChimericPair {
  pub fn identificator(&mut self, expected_tlen: i32) -> bool {
    identificator(self, expected_tlen)
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
