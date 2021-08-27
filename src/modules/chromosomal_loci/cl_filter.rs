////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use std::str::from_utf8;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  ActivateExt,
  Anchor,
  ChrAnchor,
  ChrAnchorEnum,
  MEChimericPair,
  MEChimericRead,
  OrientationEnum,
  RawValues,
  SAMFlag,
  Sequence,
  StrandDirection,
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

macro_rules! strand_direction {
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
  chr_registry: &alias::RegistryChr,
  me_record: &alias::RecordME,
  dir_registry: &alias::RegistryDir,
) {
  // iterate on registry
  // switch

  if let Some(reads_id) = chr_registry.lock().unwrap().get(ikey) {
    reads_id.iter().map(|read_id| {
      let switch_mapq = false;
      if let Some(me_pair) = me_record.lock().unwrap().get(read_id) {
        mapq(me_pair, switch_mapq);
      }

      if switch_mapq {
        purge(&me_record, read_id);
      } else {
        // segregate reads based on orientation
        // count reads

        if let Some(direction) = dir_registry.lock().unwrap().get_mut(ikey) {
          assign(&me_record, read_id, direction);
        }
      }
    });
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////

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
  me_record: &alias::RecordME,
  read_id: &str,
) {
  me_record.lock().unwrap().remove(read_id);
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn assign(
  me_record: &alias::RecordME,
  read_id: &str,
  direction: &mut StrandDirection,
) {
  if let Some(me_chimeric_pair) = me_record.lock().unwrap().get(read_id) {
    match me_chimeric_pair.chranch {
      ChrAnchorEnum::Read1 => {
        tag(&me_chimeric_pair.read1, read_id.to_string(), direction)
      }
      ChrAnchorEnum::Read2 => {
        tag(&me_chimeric_pair.read2, read_id.to_string(), direction)
      }
      ChrAnchorEnum::None => (),
    }
  }
}

fn tag(
  me_chimeric_read: &MEChimericRead,
  read_id: String,
  direction: &mut StrandDirection,
) {
  match (
    me_chimeric_read.chr_read[0].interpret(5),
    me_chimeric_read.orientation,
  ) {
    (false, OrientationEnum::Upstream) => {
      strand_direction!(direction, fs5, me_chimeric_read, read_id);
    }
    (true, OrientationEnum::Downstream) => {
      strand_direction!(direction, fs3, me_chimeric_read, read_id);
    }
    (true, OrientationEnum::Upstream) => {
      strand_direction!(direction, rs5, me_chimeric_read, read_id);
    }
    (false, OrientationEnum::Downstream) => {
      strand_direction!(direction, rs3, me_chimeric_read, read_id);
    }
    (_, _) => (),
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
