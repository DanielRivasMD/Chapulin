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
use crate::ActivateExt;
use crate::Strands;

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! strand {
  ( $strand: expr, $orientation: tt, $me_chimeric_read: expr, $read_id: expr ) => {
    $strand.$orientation.0 += 1;
    let position_vc = $strand
      .$orientation
      .1
      .entry($me_chimeric_read.chr_read[0].bin())
      .or_insert(Vec::new());
    position_vc.push($read_id);
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

const MAPQ: i32 = 20;

pub fn filter(
  ikey: &str,
  an_registry: &alias::RegistryME,
  hm_collection: &alias::RecordME,
  registry_strand: &alias::RegistryStrand,
) {
  // iterate on registry
  // switch

  if let Some(reads_id) = an_registry.lock().unwrap().get(ikey) {
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

        if let Some(strands) = registry_strand.lock().unwrap().get_mut(ikey) {
          assign(&hm_collection, read_id, strands);
        }
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
}

fn mapq(
  me_pair: &MEChimericPair,
  mut switch_mapq: bool,
) {
  match me_pair.chranch {
    ChrAnchorEnum::Read1 => {
      if me_pair.read1.chr_read[0].mapq < MAPQ {
        switch_mapq.activate();
      }
    }
    ChrAnchorEnum::Read2 => {
      if me_pair.read2.chr_read[0].mapq < MAPQ {
        switch_mapq.activate();
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
    (false, OrientationEnum::Upstream) => {
      strand!(strands, FS5, me_chimeric_read, read_id);
    }
    (true, OrientationEnum::Downstream) => {
      strand!(strands, FS3, me_chimeric_read, read_id);
    }
    (true, OrientationEnum::Upstream) => {
      strand!(strands, RS5, me_chimeric_read, read_id);
    }
    (false, OrientationEnum::Downstream) => {
      strand!(strands, RS3, me_chimeric_read, read_id);
    }
    (_, _) => (),
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
