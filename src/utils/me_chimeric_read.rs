
// crate utilities
use crate::utils::{
  me_anchor::MEAnchor,
  chr_anchor::ChrAnchor,
  break_point::BreakPoint,
};

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

impl MEChimericRead {

  // reverse complement sequence
  pub fn sequence_reverser(&self) -> String {

    self.sequence.chars()
    .map(|x| match x {
        '!' => '?',
        'A' => 'T',
        'T' => 'A',
        'C' => 'G',
        'G' => 'C',
        _ => x
    }).rev().collect()
  }

  //  TODO: add breakpoint determination as trait
}
