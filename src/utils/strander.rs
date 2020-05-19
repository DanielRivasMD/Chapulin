
// standard libraries
use std::collections::HashMap;

// crate utilities
use crate::{
  utils::{
    anchor_read::AnchorRead,
    me_read::MERead,
    mobel_counter::MobelCounter,
  },
};


pub fn strander(
  read_id: String,
  str: &str,
  chr_pair: &AnchorRead,
  me_pair: &Vec<MERead>,
  position_hm: &mut HashMap<i32, Vec<String>>
) {

  let mut mobel_counter = MobelCounter::new();

  for i in me_pair.iter() { mobel_counter.counter(&i.orientation); }

  // TODO: review carefully these conditions
  match str {
    "F5" => {
      if chr_pair.flag == 0 && mobel_counter.upstream >= mobel_counter.downstream {
        let binned_position = chr_pair.binner();
        if ! position_hm.contains_key( &binned_position) { position_hm.insert(binned_position, Vec::new()); }
        if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
      }
    },
    "F3" => {
      if chr_pair.flag == 16 && mobel_counter.upstream <= mobel_counter.downstream {
        let binned_position = chr_pair.binner();
        if ! position_hm.contains_key( &binned_position) { position_hm.insert(binned_position, Vec::new()); }
        if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
      }
    },
    "R5" => {
      if chr_pair.flag == 16 && mobel_counter.upstream >= mobel_counter.downstream {
        let binned_position = chr_pair.binner();
        if ! position_hm.contains_key( &binned_position) { position_hm.insert(binned_position, Vec::new()); }
        if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
      }
    },
    "R3" => {
      if chr_pair.flag == 0 && mobel_counter.upstream <= mobel_counter.downstream {
        let binned_position = chr_pair.binner();
        if ! position_hm.contains_key( &binned_position) { position_hm.insert(binned_position, Vec::new()); }
        if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
      }
    },
    _ => {},
  }
}
