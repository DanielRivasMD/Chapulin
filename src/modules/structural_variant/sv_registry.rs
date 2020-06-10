
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// crate utilities
use crate::{
  utils::{
    file_reader::file_reader,
    cigar::CIGAR,
    flag_interpretor::interpretor,
  },
  settings::{
    constants::ME_LIMIT,
  },
};

// place holders
use crate::utils::me_chimeric_pair::MEChimericPair;


pub fn sv_mapper(
  sv_bam_file: &String,
  expected_tlen: i32,
  hm_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

  // load file
  let (mut reader, mut buffer) = file_reader(&sv_bam_file);

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

    let mut sv_switch = true;

    if ! hm_collection.lock().unwrap().contains_key(&read_id) {
      hm_collection.lock().unwrap().insert((&read_id).to_string(), MEChimericPair::new());

      if let Some(current_record) = hm_collection.lock().unwrap().get_mut(&read_id) {
        // current_record.read1.sequence = read_seq.clone();
        // current_record.read1.me_read[0] = MEAnchor::loader(&record_line, me_size, &mobel_orientation);
      }
    } else {
      if let Some(current_record) = hm_collection.lock().unwrap().get_mut(&read_id) {
        // current_record.read2.sequence = read_seq.clone();
        // current_record.read2.me_read[0] = MEAnchor::loader(&record_line, me_size, &mobel_orientation);

        // evaluate read pairs
        // TODO: SV deletion => read with large (> 2sd observed) template length
        let tlen = current_record.read1.chr_read[0].tlen - current_record.read2.chr_read[0].tlen;
        if tlen.abs() > expected_tlen {
          sv_switch = false;
        }

        // TODO: SV duplication => read orientation reversed outwards + inverted chimerics
        if tlen > 0 && interpretor(current_record.read1.chr_read[0].flag, 10) {
          sv_switch = false;
        } else if tlen < 0 && interpretor(current_record.read2.chr_read[0].flag, 10) {
          sv_switch = false;
        } else if tlen == 0 {

        }

        // TODO: SV inversion => read orientation altered unidirectionally + inverted chimerics
        let read1_orient = interpretor(current_record.read1.chr_read[0].flag, 10);
        let read2_orient = interpretor(current_record.read2.chr_read[0].flag, 10);
        if read1_orient == read2_orient {
          sv_switch = false;
        }

        // TODO: SV insertion => unmapped reads
        if
          tlen == 0 && (
          current_record.read1.chr_read[0].pos.to_string() == "*" ||
          current_record.read2.chr_read[0].pos.to_string() == "*"
        ) {
          sv_switch = false;
        }
        // TODO: SV translocation => read mapping to other chromosomes
        if ! (current_record.read1.chr_read[0].chr == current_record.read2.chr_read[0].chr) {
          sv_switch = false;
        }

        // evaluate read batch
        if sv_switch {
          hm_collection.lock().unwrap().remove(&read_id);
        } else {
          // register chromosome anchors
          if ! an_registry.lock().unwrap().contains_key(&chr) {
            an_registry.lock().unwrap().insert(chr, Vec::new());
          }
        }
      }
    }
  }

  Ok(println!("{} {}", "File read: ", &sv_bam_file))
}
