
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::functions::{
    element_counter::ElementCounter,
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
  me_pair: &Vec<MEAnchor>,
  position_hm: &mut HashMap<i32, Vec<String>>
) -> i32 {

  let mut mobel_counter = ElementCounter::new();

  for i in me_pair.iter() { mobel_counter.counter(&i.orientation); }

  match str {
    "F5" => {
      if chr_pair.flag == 0 && mobel_counter.upstream >= mobel_counter.downstream {
        read_count = read_count + 1;
        let binned_position = chr_pair.binner();
        if ! position_hm.contains_key( &binned_position) { position_hm.insert(binned_position, Vec::new()); }
        if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
      }
    },
    "F3" => {
      if chr_pair.flag == 16 && mobel_counter.upstream <= mobel_counter.downstream {
        read_count = read_count + 1;
        let binned_position = chr_pair.binner();
        if ! position_hm.contains_key( &binned_position) { position_hm.insert(binned_position, Vec::new()); }
        if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
      }
    },
    "R5" => {
      if chr_pair.flag == 16 && mobel_counter.upstream >= mobel_counter.downstream {
        read_count = read_count + 1;
        let binned_position = chr_pair.binner();
        if ! position_hm.contains_key( &binned_position) { position_hm.insert(binned_position, Vec::new()); }
        if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
      }
    },
    "R3" => {
      if chr_pair.flag == 0 && mobel_counter.upstream <= mobel_counter.downstream {
        read_count = read_count + 1;
        let binned_position = chr_pair.binner();
        if ! position_hm.contains_key( &binned_position) { position_hm.insert(binned_position, Vec::new()); }
        if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
      }
    },
    _ => {},
  }
  return read_count
}

////////////////////////////////////////////////////////////////////////////////////////////////////
