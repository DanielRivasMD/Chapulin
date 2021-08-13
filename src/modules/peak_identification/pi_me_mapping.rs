////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  strand_count,
  threshold,
  ChrAnchorEnum,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::settings::constants::{
  NO_FDR,
  STRAND_VEC,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pi_me_identifier(
  ikey: &str,
  output: &str,
  _errata: &str,
  hm_collection: alias::RecordME,
  an_registry: alias::RegistryME,
  chr_assembly: alias::LibraryME,
) -> alias::AnyResult {
  let mut chr_position_hm = HashMap::new();
  let chr_size = *chr_assembly.lock().unwrap().get(ikey).unwrap();

  // TODO: implement parallel iteration here

  let fl_write = format!("{}{}.csv", output, ikey);
  let mut fl =
    File::create(&fl_write).context(ChapulinCommonError::CreateFile {
      f: fl_write,
    })?;

  for strand in STRAND_VEC.iter() {
    chr_position_hm.insert(strand, HashMap::new());
    let tmp_position_hm = chr_position_hm.get_mut(strand).unwrap();

    // TODO: tag orientation to reduce elements to iterate on
    // TODO: check for non-oriented mobels
    // TODO: implement a threshold selector

    let mut read_count = 0;

    let ids_read = an_registry.lock().unwrap().get(ikey).unwrap().clone();

    for id_read in ids_read {
      if let Some(me_pair) = hm_collection.lock().unwrap().get(&id_read) {
        match &me_pair.chranch {
          ChrAnchorEnum::Read1 => {
            read_count = strand_count(
              id_read,
              strand,
              read_count,
              &me_pair.read1.chr_read[0],
              &me_pair.read2.me_read,
              tmp_position_hm,
            );
          }

          ChrAnchorEnum::Read2 => {
            read_count = strand_count(
              id_read,
              strand,
              read_count,
              &me_pair.read2.chr_read[0],
              &me_pair.read1.me_read,
              tmp_position_hm,
            );
          }

          ChrAnchorEnum::None => (),
        }
      }
    }

    // TODO: memotization
    if read_count != 0 {
      let pois_threshold = threshold(
        read_count as f64,
        chr_size,
        0.001,
        tmp_position_hm,
        NO_FDR,
      );

      // TODO: verify read pairs some are empty
      // TODO: verify read passing map quality
      // TODO: verify reads no counted twice
      // TODO: format output => possibly 1) raw 2) postgreSQL

      for (chr_pos, id_vec) in tmp_position_hm.iter() {
        if id_vec.len() > pois_threshold {
          // println!();
          // println!("Position: {} @ strand: {} => {}", chr_pos, strand,
          // id_vec.len()); println!("{:?}", id_vec);

          // println!("{}, {}, {}, {}", ikey, chr_pos, strand, id_vec.len());
          let to_write =
            format!("{}, {}, {}, {}\n", ikey, chr_pos, strand, id_vec.len());

          fl.write_all(to_write.as_bytes()).context(
            ChapulinCommonError::WriteFile {
              f: to_write
            },
          )?;

          // for id_read in id_vec.iter() {
          //   if let Some((id, read)) = hm_collection
          //  .lock().unwrap()
          //  .get_key_value(id_read) {

          //     // ic!(id);
          //     // ic!(read);
          //     // ic!(read.chr_anchor_retriever());

          //     match read.chranch {
          //       ChrAnchorEnum::Read1 => println!("{} -> {}", id, read.read1),
          //       ChrAnchorEnum::Read2 => println!("{} -> {}", id, read.read2),
          //       ChrAnchorEnum::None => (),
          //     }

          //   }
          // }
        }
      }
    }
  }
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
