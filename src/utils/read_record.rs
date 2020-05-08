
// standard libraries
use std::borrow::Borrow;

// crate utilities
use crate::utils::{
  read_sequence::ReadSequence,
  chranchor_enum::ChrAnchor,
};

// to load onto => hashmap for reads aligned to mobile elements
#[derive(Debug)]
pub struct ReadRecord {
  pub read1: ReadSequence,
  pub read2: ReadSequence,
  pub chranchor: ChrAnchor,
}
// TODO: add non-cigar anchor identification

impl ReadRecord {
  pub fn new() -> Self {
    Self {
      read1: ReadSequence::new(),
      read2: ReadSequence::new(),
      chranchor: ChrAnchor::None,
    }
  }
}

impl ReadRecord {
  pub fn chr_anchor_retriever<'a>(&'a self) -> &'a ReadSequence {
    match self.chranchor {
      ChrAnchor::None => {
        // TODO: think about an alternative here
        println!("This is a default value");
        &self.read1
      },
      ChrAnchor::Read1 => &self.read1,
      ChrAnchor::Read2 => &self.read2,
    }.borrow()
  }
}
