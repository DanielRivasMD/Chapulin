
// crate utilities
use crate::utils::{
  me_anchor::MEAnchor,
  chr_anchor::ChrAnchor,
  break_point::BreakPoint,
};

// annotate primary (index 0) & secondary aligned reads
#[derive(Debug)]
pub struct MEChimericRead {
  pub sequence: String,
  pub me_read: Vec<MEAnchor>,
  pub chr_read: Vec<ChrAnchor>,
  pub breakpoint: BreakPoint,
}

impl MEChimericRead {
  pub fn new() -> Self {
    Self {
      sequence: "".to_string(),
      me_read: vec![MEAnchor::new()],
      chr_read: vec![ChrAnchor::new()],
      breakpoint: BreakPoint {
        sequence: "".to_string(),
        coordinate: 0,
      },
    }
  }
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
