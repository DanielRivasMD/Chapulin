
// crate utilities
use crate::utils::{
  chr_anchor::ChrAnchor,
  break_point::BreakPoint,
};

// annotate primary (index 0) & secondary aligned reads
#[derive(Debug, new, Default)]
pub struct SVChimericRead {
  #[new(default)]
  pub sequence: String,
  #[new(default)]
  pub chr_read: ChrAnchor,
  #[new(default)]
  // #[new(value = BreakPoint {sequence: "".to_string(), coordinate: 0})]
  pub breakpoint: BreakPoint,
}
