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
    $strand.$orientation.count += 1;
    let position_vc = $strand
      .$orientation
      .position
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
  dir_registry: &alias::RegistryDir,
  me_record: &alias::RecordME,
) {
  // iterate on registry
  // switch

  if let Some(reads_id) = chr_registry.lock().unwrap().get(ikey) {
    reads_id.iter().for_each(|read_id| {
      let mut switch_mapq = false;
      if let Some(me_pair) = me_record.lock().unwrap().get(read_id) {
        mapq(me_pair, &mut switch_mapq);
      }

      if switch_mapq {
        purge(&me_record, read_id);
      } else {
        // insert scaffold
        if !dir_registry.lock().unwrap().contains_key(ikey) {
          insert(&dir_registry, ikey);
        }

        // segregate reads based on orientation
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
  switch_mapq: &mut bool,
) {
  match me_pair.chranch {
    ChrAnchorEnum::Read1 => {
      if me_pair.read1.quality < MAPQ {
        switch_mapq.activate();
      }
    }
    ChrAnchorEnum::Read2 => {
      if me_pair.read2.quality < MAPQ {
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

fn insert(
  dir_registry: &alias::RegistryDir,
  ikey: &str,
) {
  dir_registry
    .lock()
    .unwrap()
    .insert(ikey.to_string(), StrandDirection::new());
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
        tag(
          &me_chimeric_pair.read1,
          &me_chimeric_pair.read2,
          read_id.to_string(),
          direction,
        )
      }
      ChrAnchorEnum::Read2 => {
        tag(
          &me_chimeric_pair.read2,
          &me_chimeric_pair.read1,
          read_id.to_string(),
          direction,
        )
      }
      ChrAnchorEnum::None => (),
    }
  }
}

fn tag(
  chimeric_chr_read: &MEChimericRead,
  chimeric_me_read: &MEChimericRead,
  read_id: String,
  direction: &mut StrandDirection,
) {
  match (
    chimeric_chr_read.chr_read[0].interpret(5),
    chimeric_me_read.orientation,
  ) {
    (false, OrientationEnum::Upstream) => {
      strand_direction!(direction, fs5, chimeric_chr_read, read_id);
    }
    (true, OrientationEnum::Downstream) => {
      strand_direction!(direction, fs3, chimeric_chr_read, read_id);
    }
    (true, OrientationEnum::Upstream) => {
      strand_direction!(direction, rs5, chimeric_chr_read, read_id);
    }
    (false, OrientationEnum::Downstream) => {
      strand_direction!(direction, rs3, chimeric_chr_read, read_id);
    }
    (_, _) => (),
  }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
