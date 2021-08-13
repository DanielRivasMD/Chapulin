////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::threshold;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::settings::constants::NO_FDR;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pi_sv_identifier(
  ikey: &str,
  _hm_collection: alias::RecordSV,
  an_registry: alias::RegistryME,
  chr_assembly: alias::LibraryME,
) -> alias::AnyResult {
  let chr_size = *chr_assembly.lock().unwrap().get(ikey).unwrap();
  let ids_read = an_registry.lock().unwrap().get(ikey).unwrap().clone();

  let read_count = ids_read.len();

  let chr_position_hm = HashMap::new();

  // // TODO: work on this macro
  // for id_read in ids_read {
  //   if let Some(sv_pair) = hm_collection
  //     .lock().unwrap()
  //     .get(&id_read) {
  //     count_chr!(
  //       id_read,
  //       sv_pair,
  //       &mut chr_position_hm
  //     );
  //   }
  // }

  // TODO: memotization
  if read_count != 0 {
    let pois_threshold =
      threshold(read_count as f64, chr_size, 0.001, &chr_position_hm, NO_FDR);

    for (chr_pos, id_vec) in chr_position_hm.iter() {
      if id_vec.len() > pois_threshold {
        println!("{}, {}, {}", ikey, chr_pos, id_vec.len());
      }
    }
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
