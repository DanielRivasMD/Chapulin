////////////////////////////////////////////////////////////////////////////////////////////////////

// // standard libraries
// use anyhow::Context;
use std::collections::HashMap;
// use std::fs::File;
// use std::io::Write;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  strand_count,
  threshold,
  Anchor,
  ChrAnchorEnum,
  MEChimericPair,
  MEChimericRead,
  OrientationEnum,
  SAMFlag,
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

const MAPQ: i32 = 20;

pub fn pi_me_identifier(
  ikey: &str,
  output: &str,
  _errata: &str,
  an_registry: alias::RegistryME,
  chr_assembly: alias::LibraryME,
  hm_collection: alias::RecordME,
) -> alias::AnyResult {
  // let mut chr_position_hm = HashMap::new();
  // let chr_size = *chr_assembly.lock().unwrap().get(ikey).unwrap();

  // declare strand reference
  let mut strands = Strands::new();

  // TODO: implement parallel iteration here

  // filter hits
  //////////////////////////////////////////////////
  // select based on likehood of alignment -> MAPQ
  //////////////////////////////////////////////////

  if let Some(reads_id) = an_registry.lock().unwrap().get(ikey) {
    filter(reads_id, &hm_collection, &mut strands);
  }

  // filter based on estimated & false discovery rate threshold
  //////////////////////////////////////////////////

  for strand in STRAND_VEC.iter() {
    // fdr(&strands, &hm_collection);
  }

  // write results
  //////////////////////////////////////////////////

  // let fl_write = format!("{}{}.csv", output, ikey);
  // let mut fl =
  //   File::create(&fl_write).context(ChapulinCommonError::CreateFile {
  //     f: fl_write,
  //   })?;

  // for strand in STRAND_VEC.iter() {
  //   chr_position_hm.insert(strand, HashMap::new());
  //   let tmp_position_hm = chr_position_hm.get_mut(strand).unwrap();

  //   // TODO: tag orientation to reduce elements to iterate on
  //   // TODO: check for non-oriented mobels
  //   // TODO: implement a threshold selector

  //   let mut read_count = 0;

  //   let ids_read = an_registry.lock().unwrap().get(ikey).unwrap().clone();

  //   for id_read in ids_read {
  //     if let Some(me_pair) = hm_collection.lock().unwrap().get(&id_read) {
  //       match &me_pair.chranch {
  //         ChrAnchorEnum::Read1 => {
  //           read_count = strand_count(
  //             id_read,
  //             strand,
  //             read_count,
  //             &me_pair.read1.chr_read[0],
  //             &me_pair.read2.me_read,
  //             tmp_position_hm,
  //           );
  //         }

  //         ChrAnchorEnum::Read2 => {
  //           read_count = strand_count(
  //             id_read,
  //             strand,
  //             read_count,
  //             &me_pair.read2.chr_read[0],
  //             &me_pair.read1.me_read,
  //             tmp_position_hm,
  //           );
  //         }

  //         ChrAnchorEnum::None => (),
  //       }
  //     }
  //   }

  //   // TODO: memotization
  //   if read_count != 0 {
  //     let pois_threshold =
  //       threshold(read_count as f64, chr_size, 0.001, tmp_position_hm,
  // NO_FDR);

  //     // TODO: verify read pairs some are empty
  //     // TODO: verify read passing map quality
  //     // TODO: verify reads no counted twice
  //     // TODO: format output => possibly 1) raw 2) postgreSQL

  //     for (chr_pos, id_vec) in tmp_position_hm.iter() {
  //       if id_vec.len() > pois_threshold {
  //         // println!();
  //         // println!("Position: {} @ strand: {} => {}", chr_pos, strand,
  //         // id_vec.len()); println!("{:?}", id_vec);

  //         // println!("{}, {}, {}, {}", ikey, chr_pos, strand, id_vec.len());
  //         let to_write =
  //           format!("{}, {}, {}, {}\n", ikey, chr_pos, strand, id_vec.len());

  //         fl.write_all(to_write.as_bytes()).context(
  //           ChapulinCommonError::WriteFile {
  //             f: to_write
  //           },
  //         )?;

  //         // for id_read in id_vec.iter() {
  //         //   if let Some((id, read)) = hm_collection
  //         //  .lock().unwrap()
  //         //  .get_key_value(id_read) {

  //         //     // dbg!(id);
  //         //     // dbg!(read);
  //         //     // dbg!(read.chr_anchor_retriever());

  //         //     match read.chranch {
  //         //       ChrAnchorEnum::Read1 => println!("{} -> {}", id,
  // read.read1),         //       ChrAnchorEnum::Read2 => println!("{} ->
  // {}", id, read.read2),         //       ChrAnchorEnum::None => (),
  //         //     }

  //         //   }
  //         // }
  //       }
  //     }
  //   }
  // }

  Ok(())
}

// println!("Result from MAPQ: {:?}", self.anchor(hm_record_collection));
// if self.anchor(hm_record_collection) {
//   // if self.read_id.current == "SRR556146.17" {
//   //   println!("Removing");
//   //   println!();
//   // }
//   hm_record_collection
//     .lock()
//     .unwrap()
//     .remove(&self.read_id.current);
// } else {
//   // if self.read_id.current == "SRR556146.17" {
//   //   println!("Registering");
//   //   println!();
//   // }

////////////////////////////////////////////////////////////////////////////////////////////////////

// trait RegisterExt {
// fn anchor(
//   &self,
//   hm_record_collection: &alias::RecordME,
// ) -> bool;
// }

////////////////////////////////////////////////////////////////////////////////////////////////////

// // impl RegisterExt for RawValues {
// fn anchor(
//   &self,
//   hm_record_collection: &alias::RecordME,
// ) -> bool {
//   let mut switch_out = true;
//   if let Some(current_record) = hm_record_collection
//     .lock()
//     .unwrap()
//     .get(&self.read_id.current)
//   {
//     // println!("{:#?}", current_record);
//     match current_record.chranch {
//       ChrAnchorEnum::Read1 => {
//         println!(
//           "Record chr is empty: {:?}",
//           current_record.read1.chr_read.is_empty()
//         );
//         switch_out = mapq!(current_record, read1);
//       }
//       ChrAnchorEnum::Read2 => {
//         switch_out = mapq!(current_record, read2);
//         // println!("Inside Match");
//         // println!("{:?}", current_record.read2.chr_read.is_empty());
//         // println!("{:?}", current_record.read2.chr_read[0].mapq < MAPQ);
//         // println!("{:?}", switch_out);
//       }
//       _ => (),
//     };
//   }

//   // if self.read_id.current == "SRR556146.17" {
//   //   println!("Inside Match");
//   //   //   println!();
//   //   //   println!("{:#?}", self);
//   //   //   println!("Switch: {:?}", switch_out);
//   // }
//   switch_out
// }
// // }

////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, new, Default)]
struct Strands {
  #[new(default)]
  FS5: Vec<String>,

  #[new(default)]
  FS3: Vec<String>,

  #[new(default)]
  RS5: Vec<String>,

  #[new(default)]
  RS3: Vec<String>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn filter(
  reads_id: &Vec<String>,
  hm_collection: &alias::RecordME,
  strands: &mut Strands,
) {
  // iterate on registry
  // switch

  reads_id.iter().map(|read_id| {
    let switch_mapq = true;
    if let Some(me_pair) = hm_collection.lock().unwrap().get(read_id) {
      mapq(me_pair, switch_mapq);
    }

    if switch_mapq {
      purge(&hm_collection, read_id);
    } else {
      // segregate reads based on orientation
      // count reads

      assign(&hm_collection, read_id, strands);
    }

    // switch_mapq = false;

    //   match &me_pair.chranch {
    //     ChrAnchorEnum::Read1 => {
    //       read_count = strand_count(
    //         id_read,
    //         strand,
    //         read_count,
    //         &me_pair.read1.chr_read[0],
    //         &me_pair.read2.me_read,
    //         tmp_position_hm,
    //       );
    //     }

    //     ChrAnchorEnum::Read2 => {
    //       read_count = strand_count(
    //         id_read,
    //         strand,
    //         read_count,
    //         &me_pair.read2.chr_read[0],
    //         &me_pair.read1.me_read,
    //         tmp_position_hm,
    //       );
    //     }

    //     ChrAnchorEnum::None => (),
    //   }
    // }
    // }
  });
  // }
}

fn mapq(
  me_pair: &MEChimericPair,
  mut switch_mapq: bool,
) {
  match me_pair.chranch {
    ChrAnchorEnum::Read1 => {
      if me_pair.read1.chr_read[0].mapq < MAPQ {
        switch_mapq = true;
      }
    }
    ChrAnchorEnum::Read2 => {
      if me_pair.read2.chr_read[0].mapq < MAPQ {
        switch_mapq = true;
      }
    }
    ChrAnchorEnum::None => (),
  }
}

fn purge(
  hm_collection: &alias::RecordME,
  read_id: &str,
) {
  hm_collection.lock().unwrap().remove(read_id);
}

fn assign(
  hm_collection: &alias::RecordME,
  read_id: &str,
  strands: &mut Strands,
) {
  if let Some(me_chimeric_pair) = hm_collection.lock().unwrap().get(read_id) {
    match me_chimeric_pair.chranch {
      ChrAnchorEnum::Read1 => {
        tag(&me_chimeric_pair.read1, read_id.to_string(), strands)
      }
      ChrAnchorEnum::Read2 => {
        tag(&me_chimeric_pair.read2, read_id.to_string(), strands)
      }
      ChrAnchorEnum::None => (),
    }
  }
}

fn tag(
  me_chimeric_read: &MEChimericRead,
  read_id: String,
  strands: &mut Strands,
) {
  match (
    me_chimeric_read.chr_read[0].interpret(5),
    me_chimeric_read.orientation,
  ) {
    (false, OrientationEnum::Upstream) => strands.FS5.push(read_id),
    (true, OrientationEnum::Downstream) => strands.FS3.push(read_id),
    (true, OrientationEnum::Upstream) => strands.RS5.push(read_id),
    (false, OrientationEnum::Downstream) => strands.RS3.push(read_id),
    (_, _) => (),
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn fdr() {
  unimplemented!();
}

fn count(
  strands: &Strands,
  hm_collection: &alias::RecordME,
) -> HashMap<i32, i32> {
  // for strand in STRAND_VEC.iter() {

  let strand = &strands.FS5;
  let mut chr_position_hm = HashMap::new();

  strand.iter().for_each(|read_id| {
    if let Some(current_record) = hm_collection.lock().unwrap().get(read_id) {
      match current_record.chranch {
        ChrAnchorEnum::Read1 => {
          let bin_position = current_record.read1.chr_read[0].bin();
          if !chr_position_hm.contains_key(&bin_position) {
            chr_position_hm.insert(bin_position, 1);
          } else {
            if let Some(current_position) =
              chr_position_hm.get_mut(&bin_position)
            {
              *current_position += 1;
            }
          }
        }
        ChrAnchorEnum::Read2 => (), // TODO: complete
        _ => (),
      }
    }
  });
  // }
  return chr_position_hm;
}

////////////////////////////////////////////////////////////////////////////////////////////////////
