////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use std::str::from_utf8;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  Anchor,
  ChrAnchor,
  ChrAnchorEnum,
  MEChimericPair,
  MEChimericRead,
  OrientationEnum,
  RawValues,
  SAMFlag,
  Sequence,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::utils::io::file_reader::byte_file_reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate features
use crate::Strands;

////////////////////////////////////////////////////////////////////////////////////////////////////

const MAPQ: i32 = 20;

pub fn filter(
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
