
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::functions::{
    strander::strander,
    thresholder::thresholder,
  },
  utils::structures::{
    sv_chimeric_pair::SVChimericPair,
    chr_anchor_enum::ChrAnchorEnum,
  },
  settings::{
    constants::{
      STRAND_VEC,
      NO_FDR,
    },
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
// use crate::error::{
// }

////////////////////////////////////////////////////////////////////////////////////////////////////


// TODO: make this module only dedicated to peak thresholding & detection for SV compatibility
pub fn pi_sv_identifier (
  ikey: &str,
  hm_collection: Arc<Mutex<HashMap<String, SVChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
  chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {

  ic!(ikey);

  // let mut chr_position_hm = HashMap::new();
  let chr_size = *chr_assembly.lock().expect("chromosome not found").get(ikey).unwrap();

// TODO: iterate through chromosome binning positions & count

// TODO: memotization
    if read_count != 0 {
      let pois_threshold = thresholder(
        read_count as f64,
        chr_size,
        0.001,
        tmp_position_hm,
        NO_FDR,
      );


      for (chr_pos, id_vec) in tmp_position_hm.iter() {

        if id_vec.len() > pois_threshold {
          println!();
          println!("{}, {}, {}", ikey, chr_pos, id_vec.len());

            }
          }
        }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
