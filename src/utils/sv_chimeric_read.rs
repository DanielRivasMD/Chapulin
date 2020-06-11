
// crate utilities
use crate::utils::{
  chr_anchor::ChrAnchor,
  break_point::BreakPoint,
};

// annotate primary (index 0) & secondary aligned reads
#[derive(Debug)]
pub struct SVChimericRead {
  pub sequence: String,
  pub chr_read: ChrAnchor,
  pub breakpoint: BreakPoint,
}

impl SVChimericRead {
  pub fn new() -> Self {
    Self {
      sequence: "".to_string(),
      chr_read: ChrAnchor::new(),
      breakpoint: BreakPoint {
        sequence: "".to_string(),
        coordinate: 0,
      },
    }
  }

  //  TODO: add breakpoint determination as trait
}
