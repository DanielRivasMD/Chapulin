
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn chr_counter(
  read_id: String,
  position_hm: &mut HashMap<String, Vec<String>>,
  binned_position: String,
) {
  position_hm.entry(binned_position.clone()).or_insert_with(Vec::new);
  if let Some(id_vector) = position_hm.get_mut( &binned_position) { id_vector.push(read_id); }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
