
// standard libraries
use std::borrow::Borrow;

// crate utilities
use crate::utils::{
  read_sequence::ReadSequence,
  anchor_enum::Anchor,
};

// to load onto => hashmap for reads aligned to mobile elements
#[derive(Debug)]
pub struct ReadRecord {
  pub read1: ReadSequence,
  pub read2: ReadSequence,
  pub anchor: Anchor,
  pub debug_seq: String,
}

impl ReadRecord {
  pub fn new() -> Self {
    Self {
      read1: ReadSequence::new(),
      read2: ReadSequence::new(),
      anchor: Anchor::ReadDefault,
      debug_seq: "".to_string(),
    }
  }
}

impl ReadRecord {
  pub fn chr_anchor_retriever<'a>(&'a self) -> &'a ReadSequence {
    match self.anchor {
      Anchor::ReadDefault => {
        println!("This is a default value");
        &self.read1
      },
      Anchor::Read1 => &self.read1,
      Anchor::Read2 => &self.read2,
    }.borrow()
  }
}

