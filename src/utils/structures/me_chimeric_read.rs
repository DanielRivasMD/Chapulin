
////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::structures::{
    me_anchor::MEAnchor,
    chr_anchor::ChrAnchor,
    break_point::BreakPoint,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// annotate primary (index 0) & secondary aligned reads
#[derive(Debug, new, Default)]
pub struct MEChimericRead {

  #[new(default)]
  pub sequence: String,

  #[new(default)]
  pub me_read: Vec<MEAnchor>,

  #[new(default)]
  pub chr_read: Vec<ChrAnchor>,

  #[new(default)]
  pub breakpoint: BreakPoint,

}

////////////////////////////////////////////////////////////////////////////////////////////////////

impl MEChimericRead {

  // reverse complement sequence
  pub fn sequence_reverser(&self) -> String {

    self.sequence
      .chars()
      .map(|x| match x {
          '!' => '?',
          'A' => 'T',
          'T' => 'A',
          'C' => 'G',
          'G' => 'C',
          _ => x
      })
      .rev()
      .collect()
  }

  //  TODO: add breakpoint determination as trait
}

////////////////////////////////////////////////////////////////////////////////////////////////////

use std::fmt;

impl fmt::Display for MEChimericRead {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

    writeln!(
      f,
      "Chromosome: {}, Position: {}",
      self.chr_read[0].chr,
      self.chr_read[0].pos,
    )
  }
}

  // #[new(default)]
  // pub sequence: String,
  // #[new(default)]
  // pub me_read: Vec<MEAnchor>,
  // #[new(default)]
  // pub chr_read: Vec<ChrAnchor>,
  // #[new(default)]
  // pub breakpoint: BreakPoint,

////////////////////////////////////////////////////////////////////////////////////////////////////
