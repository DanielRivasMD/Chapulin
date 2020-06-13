
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// crate utilities
use crate::{
  utils::{
    file_reader::file_reader,
    cigar::CIGAR,
    flag_interpretor::interpretor,
    sv_chimeric_pair::SVChimericPair,
    chr_anchor::ChrAnchor,
    sv_type::SVType,
  },
  settings::{
    constants::ME_LIMIT,
  },
};


pub fn sv_mapper(
  sv_bam_file: &String,
  expected_tlen: i32,
  hm_collection: Arc<Mutex<HashMap<String, SVChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

  // load file
  let (mut reader, mut buffer) = file_reader(&sv_bam_file);

  // declare initial values
  let mut prev_read_id = String::new();
  let mut purge_switch = true;

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    // update read id
    let read_id = record_line[0].to_string();

    // calculate current values
    let chr = record_line[2].to_string();
    let read_seq = record_line[9].to_string();

    // flag & read orientation
    let flag = record_line[1].parse::<i32>().unwrap();
    // let read_orientation = interpretor(flag, 5);

    // alignment interpretation
    let position = record_line[3].parse::<i32>().unwrap();
    let cigar = record_line[5].to_string();
    let dc_cigar = CIGAR::loader(&cigar);
    let adj_left_pos = dc_cigar.left_boundry(position);
    let adj_right_pos = dc_cigar.right_boundry(position);

    // purge read pairs
    if ! ( prev_read_id == read_id || prev_read_id == "".to_string() ) {
      // evaluate read batch
      if purge_switch {
          // println!("Deleting: {}", prev_read_id);
          hm_collection.lock().unwrap().remove(&prev_read_id);
          // hm_record_collection.remove(&read_id);
      } else {
        // println!("to keep: {}", &read_id);
        // register chromosome anchors
        if ! an_registry.lock().unwrap().contains_key(&chr) {
          an_registry.lock().unwrap().insert(chr, Vec::new());
        }
      }

      // reset purge switch
      purge_switch = true;
    }

    if ! hm_collection.lock().unwrap().contains_key(&read_id) {
      hm_collection.lock().unwrap().insert((&read_id).to_string(), SVChimericPair::new());

      if let Some(current_record) = hm_collection.lock().unwrap().get_mut(&read_id) {
        // println!("read 1: {:?}", current_record);
        current_record.read1.sequence = read_seq.clone();
        current_record.read1.chr_read = ChrAnchor::loader(&record_line);
      }
    } else {
      if let Some(current_record) = hm_collection.lock().unwrap().get_mut(&read_id) {


        // println!("read 2: {:?}", current_record);
        current_record.read2.sequence = read_seq.clone();
        current_record.read2.chr_read = ChrAnchor::loader(&record_line);

        // evaluate read pairs
        // TODO: consider using match on expression => efficiency
        // TODO: SV deletion => read with large (> 2sd observed) template length
        // println!("{:#?}", current_record);
        // println!("R1: {}\tR2: {}", current_record.read1.chr_read.tlen, current_record.read2.chr_read.tlen);
        // println!("R1: {}\tR2: {}", current_record.read1.chr_read.pos, current_record.read2.chr_read.pos);
        let tlen = current_record.read1.chr_read.pos - current_record.read2.chr_read.pos;
        // let x = tlen.abs() > expected_tlen;
        // if x {
        if tlen.abs() > expected_tlen {

        // println!("observed tlen: {}, expected tlen: {}", tlen, expected_tlen);
        // println!("absolute: {}", tlen.abs());

          current_record.svtag = SVType::Deletion;
          purge_switch = false;
        }
        // println!("svdeletion: {}\tsvswitch: {}", x, sv_switch);

        // // TODO: SV duplication => read orientation reversed outwards + inverted chimerics
        // if tlen > 0 && interpretor(current_record.read1.chr_read.flag, 10) {
        //   sv_switch = false;
        // } else if tlen < 0 && interpretor(current_record.read2.chr_read.flag, 10) {
        //   sv_switch = false;
        // } else if tlen == 0 {
        //
        // }


        // // TODO: SV inversion => read orientation altered unidirectionally + inverted chimerics
        if interpretor(current_record.read1.chr_read.flag, 5) == interpretor(current_record.read1.chr_read.flag, 6) {
          current_record.svtag = SVType::Inversion;
          purge_switch = false;
        }
        // let read1_orient = interpretor(current_record.read1.chr_read.flag, 10);
        // let read2_orient = interpretor(current_record.read2.chr_read.flag, 10);
        // if read1_orient == read2_orient {
        //   sv_switch = false;
        // }
        //
        // TODO: SV insertion => unmapped reads
        // if
        //   tlen == 0 && (
        //   current_record.read1.chr_read.pos.to_string() == "*" ||
        //   current_record.read2.chr_read.pos.to_string() == "*"
        // ) {
        //   sv_switch = false;
        // }
        if interpretor(current_record.read1.chr_read.flag, 3) | interpretor(current_record.read1.chr_read.flag, 4) {
          current_record.svtag = SVType::Insetion;
          purge_switch = false;
        }
        // // TODO: SV translocation => read mapping to other chromosomes
        // if ! (current_record.read1.chr_read.chr == current_record.read2.chr_read.chr) {
        //   sv_switch = false;
        // }

        if
          tlen.abs() > TRANSLOCATION_DISTANCE |
        ! (current_record.read1.chr_read.chr == current_record.read2.chr_read.chr) {
          current_record.svtag == SVType::Translocation;
          purge_switch = false;
        }

        // // debug sv
        // sv_switch = false;

        // evaluate read batch
      }
    }
    prev_read_id = read_id;
  }

  // evaluate at end of file
  if purge_switch {
          // println!("Last check: {:?}", hm_record_collection.lock().unwrap().get(&prev_read_id));

    hm_collection.lock().unwrap().remove(&prev_read_id);
    // hm_record_collection.remove(&read_id);
  }

  Ok(println!("{} {}", "File read: ", &sv_bam_file))
}
