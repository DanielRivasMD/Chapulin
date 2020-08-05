
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::str::{from_utf8};
use anyhow::{Context};
use anyhow::Result as anyResult;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  utils::functions::{
    file_reader::byte_file_reader,
    flag_interpretor::interpretor,
    flag_interpretor::SamFlag,
  },
  utils::structures::{
    // cigar::CIGAR,
    sv_chimeric_pair::SVChimericPair,
    chr_anchor::ChrAnchor,
    sv_type::SVType,
  },
  settings::{
    constants::TRANSLOCATION_DISTANCE,
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  // sv_error::ChapulinSVError,
  common_error::ChapulinCommonError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn sv_mapper(
  sv_bam_file: &str,
  expected_tlen: i32,
  hm_collection: Arc<Mutex<HashMap<String, SVChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> anyResult<()> {

  // load file
  // let (mut reader, mut buffer) = buff_file_reader(&sv_bam_file);
  let mut lines = byte_file_reader(&sv_bam_file)?;

  // declare initial values
  let mut prev_read_id = String::new();
  let mut purge_switch = true;

  // iterate through file
  // while let Some(line) = reader.read_line(&mut buffer) {
  while let Some(line) = lines.next() {

    // let record_line: Vec<&str> = line?.trim().split("\t").collect();
    let record_line: Vec<&str> = from_utf8(&line?)
      .context(ChapulinCommonError::RegistryLine)?
      .trim()
      .split('\t')
      .collect();

    // update read id
    let read_id = record_line[0].to_string();

    // calculate current values
    let chr = record_line[2].to_string();
    let read_seq = record_line[9].to_string();

    // // flag & read orientation
    // let flag = record_line[1].parse::<i32>().unwrap();
    // let read_orientation = interpretor(flag, 5);

    // // alignment interpretation
    // let position = record_line[3].parse::<i32>().unwrap();
    // let cigar = record_line[5].to_string();
    // let dc_cigar = CIGAR::loader(&cigar);
    // let adj_left_pos = dc_cigar.left_boundry(position);
    // let adj_right_pos = dc_cigar.right_boundry(position);

    // purge read pairs
    if ! ( prev_read_id == read_id || prev_read_id == "" ) {
      // evaluate read batch
      if purge_switch {
          hm_collection.lock().unwrap().remove(&prev_read_id);
          // hm_record_collection.remove(&read_id);
      } else {
        // register chromosome anchors
        if ! an_registry.lock().unwrap().contains_key(&chr) {
          an_registry.lock().unwrap().insert(chr, Vec::new());
        }
      }

      // reset purge switch
      purge_switch = true;
    }

    if ! hm_collection.lock().unwrap().contains_key(&read_id) {
      hm_collection.lock().unwrap().insert((&read_id).to_string(), SVChimericPair::new(SVType::None));

      if let Some(current_record) = hm_collection.lock().unwrap().get_mut(&read_id) {
        current_record.read1.sequence = read_seq.clone();
        current_record.read1.chr_read = ChrAnchor::loader(&record_line);
      }
    } else if let Some(current_record) = hm_collection.lock().unwrap().get_mut(&read_id) {

      current_record.read2.sequence = read_seq.clone();
      current_record.read2.chr_read = ChrAnchor::loader(&record_line);

      // evaluate read pairs
      // TODO: SV deletion => read with large (> 2sd observed) template length
      let tlen = current_record.read1.chr_read.pos - current_record.read2.chr_read.pos;
      if tlen.abs() > expected_tlen {
        current_record.svtag = SVType::Deletion;
        purge_switch = false;
      }

      // TODO: SV duplication => read orientation reversed outwards + inverted chimerics
      if current_record.read1.chr_read.tlen > 0 && ! interpretor(current_record.read1.chr_read.flag, 5) && ! interpretor(current_record.read2.chr_read.flag, 5) {
        current_record.svtag = SVType::Duplication;
        purge_switch = false;
      }

      // TODO: SV inversion => read orientation altered unidirectionally + inverted chimerics
      if interpretor(current_record.read1.chr_read.flag, 5) == interpretor(current_record.read2.chr_read.flag, 5) && (current_record.read1.chr_read.chr == current_record.read2.chr_read.chr) {
        current_record.svtag = SVType::Inversion;
        purge_switch = false;
      }

      // TODO: SV insertion => unmapped reads

      current_record.read1.chr_read.interpretor(3);
      if interpretor(current_record.read1.chr_read.flag, 3) || interpretor(current_record.read2.chr_read.flag, 3) {
        current_record.svtag = SVType::Insertion;
        purge_switch = false;
      }

      // TODO: SV translocation => read mapping to other chromosomes
      if
        tlen.abs() > TRANSLOCATION_DISTANCE || current_record.read1.chr_read.chr != current_record.read2.chr_read.chr {
        current_record.svtag = SVType::Translocation;
        purge_switch = false;
      }
    }
    prev_read_id = read_id;
  }

  // evaluate at end of file
  if purge_switch {
    hm_collection.lock().unwrap().remove(&prev_read_id);
  }

  println!("File read: {}", &sv_bam_file);
  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
