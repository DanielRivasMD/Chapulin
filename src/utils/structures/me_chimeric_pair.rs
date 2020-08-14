
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::borrow::Borrow;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::structures::{
    me_chimeric_read::MEChimericRead,
    chr_anchor_enum::ChrAnchorEnum,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// to load onto => hashmap for reads aligned to mobile elements
#[derive(Debug, new)]
pub struct MEChimericPair {

  #[new(default)]
  pub read1: MEChimericRead,

  #[new(default)]
  pub read2: MEChimericRead,

  pub chranch: ChrAnchorEnum,

}
// TODO: add non-cigar anchor identification

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MEChimericPair {
  pub fn chr_anchor_retriever(&self) -> &MEChimericRead {
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

////////////////////////////////////////////////////////////////////////////////////////////////////
