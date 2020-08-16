
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::functions::{
    element_counter::ElementCounter,
    chr_counter::chr_counter,
  },
  utils::structures::{
    chr_anchor::ChrAnchor,
    me_anchor::MEAnchor,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn strander(
  read_id: String,
  str: &str,
  mut read_count: i32,
  chr_pair: &ChrAnchor,
  me_pair: &[MEAnchor],
  position_hm: &mut HashMap<String, Vec<String>>
) -> i32 {

  let mut mobel_counter = ElementCounter::new();

  for i in me_pair.iter() { mobel_counter.counter(&i.orientation); }

  match str {
    "F5" => {
      if chr_pair.flag == 0 && mobel_counter.upstream >= mobel_counter.downstream {
        chr_counter!(
          read_id,
          read_count,
          chr_pair,
          position_hm
        );
      }
    },
    "F3" => {
      if chr_pair.flag == 16 && mobel_counter.upstream <= mobel_counter.downstream {
        chr_counter!(
          read_id,
          read_count,
          chr_pair,
          position_hm
        );
      }
    },
    "R5" => {
      if chr_pair.flag == 16 && mobel_counter.upstream >= mobel_counter.downstream {
        chr_counter!(
          read_id,
          read_count,
          chr_pair,
          position_hm
        );
      }
    },
    "R3" => {
      if chr_pair.flag == 0 && mobel_counter.upstream <= mobel_counter.downstream {
        chr_counter!(
          read_id,
          read_count,
          chr_pair,
          position_hm
        );
      }
    },
    _ => {},
  }
  read_count
}

////////////////////////////////////////////////////////////////////////////////////////////////////
