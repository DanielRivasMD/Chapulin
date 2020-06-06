
// standard libraries
use std::borrow::Borrow;

// crate utilities
use crate::utils::{
  me_chimeric_read::MEChimericRead,
  chr_anchor_enum::ChrAnchorEnum,
};

// to load onto => hashmap for reads aligned to mobile elements
#[derive(Debug)]
pub struct MEChimericPair {
  pub read1: MEChimericRead,
  pub read2: MEChimericRead,
  pub chranch: ChrAnchorEnum,
}
// TODO: add non-cigar anchor identification

impl MEChimericPair {
  pub fn new() -> Self {
    Self {
      read1: MEChimericRead::new(),
      read2: MEChimericRead::new(),
      chranch: ChrAnchorEnum::None,
    }
  }
}

impl MEChimericPair {
  pub fn chr_anchor_retriever<'a>(&'a self) -> &'a MEChimericRead {
    match self.chranch {
      ChrAnchorEnum::None => {
        // TODO: think about an alternative here
        println!("This is a default value");
        &self.read1
      },
      ChrAnchorEnum::Read1 => &self.read1,
      ChrAnchorEnum::Read2 => &self.read2,
    }.borrow()
  }
}
