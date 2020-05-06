
// crate utilities
use crate::utils::{
  me_read::MERead,
  anchor_read::AnchorRead,
  break_point::BreakPoint,
};

// annotate primary (index 0) & secondary aligned reads
#[derive(Debug)]
pub struct ReadSequence {
  pub sequence: String,
  pub me_read: Vec<MERead>,
  pub chr_read: Vec<AnchorRead>,
  pub breakpoint: BreakPoint,
}

impl ReadSequence {
  pub fn new() -> Self {
    Self {
      sequence: "".to_string(),
      me_read: vec![MERead::new()],
      chr_read: vec![AnchorRead::new()],
      breakpoint: BreakPoint {
        sequence: "".to_string(),
        coordinate: 0,
      },
    }
  }
}

impl ReadSequence {

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
