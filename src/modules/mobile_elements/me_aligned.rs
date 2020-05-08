
// standard libraries
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex
};

// crate utilities
use crate::{
  utils::{
    file_reader,
    read_record::ReadRecord,
    me_library::MElibrary,
    me_read::MERead,
    cigar::CIGAR,
    anchor_enum::ChrAnchor,
    flag_interpretor::*,
  },
  settings::{
    constants::*,
  },
};

pub fn me_identificator(
  me_bam_file: &String,
  hm_record_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  hm_me_collection: &HashMap<String, MElibrary>,
) -> std::io::Result<()> {

// pub fn me_identificator(
//   me_bam_file: &String,
//   hm_record_collection: &mut HashMap<String, ReadRecord>,
//   hm_me_collection: &HashMap<String, MElibrary>,
// ) -> std::io::Result<()> {

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&me_bam_file);

  // declare initial values
  let mut read_id = String::new();
  let mut purge_switch = true;
  let mut mobel_anchor = false;
  let mut me_size = 0;
  let mut bk_count = 0;

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    // load line into vector
    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    // purge read pairs
    if ! ( read_id == record_line[0].to_string() || read_id == "".to_string() ) {

      // evaluate read batch
      if purge_switch {
        hm_record_collection.lock().unwrap().remove(&read_id);
        // hm_record_collection.remove(&read_id);
      }

      // reset purge switch
      purge_switch = true;
    }

    // update read id
    read_id = record_line[0].to_string();

    // calculate current values
    let mobel = record_line[2].to_string();
    let read_seq = record_line[9].to_string();

    // flag & read orientation
    let pv_flag = record_line[1].parse::<i32>().unwrap();
    let read_orientation = interpretor(pv_flag, 5);

    // alignment interpretation
    let pv_position = record_line[3].parse::<i32>().unwrap();
    let pv_cigar = record_line[5].to_string();
    let dc_cigar = CIGAR::loader(&pv_cigar);
    let adj_left_pos = dc_cigar.left_boundry(pv_position);
    let adj_right_pos = dc_cigar.right_boundry(pv_position);

    // TODO: describe break point signature
    // TODO: define filters for keeping based on breakpoint estimation & read orientation

    // retrieve mobile element library records
    let me_option = hm_me_collection.get(&mobel);
    match me_option {
      Some(me_record) => {
        me_size = me_record.me_size;
      },
      None => (),
    }

    // read pair selection criteria
    if
      ( adj_left_pos <= ME_LIMIT &&
        read_orientation ) ||
      ( adj_right_pos >= me_size - ME_LIMIT &&
        ! read_orientation )
    {
      // tagging
      purge_switch = false;
      mobel_anchor = true;
    }

    // match on proviral flag
    match pv_flag { // this check is much faster than using binary interpretor

      // primary alignment
      pf if pf <= 255 => {

        if ! hm_record_collection.lock().unwrap().contains_key(&read_id) {
        // if ! hm_record_collection.contains_key(&read_id) {
          hm_record_collection.lock().unwrap().insert((&read_id).to_string(), ReadRecord::new());
          // hm_record_collection.insert((&read_id).to_string(), ReadRecord::new());

          if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
          // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
            current_record.read1.sequence = read_seq.clone();
            current_record.read1.me_read[0] = MERead::loader(&record_line, me_size);
            if mobel_anchor {
              current_record.chranchor = ChrAnchor::Read2;
            }

            // record break point signature
            if
              ( adj_left_pos < 1 ) ||
              ( adj_right_pos > me_size )
            {
              bk_count = 1 + bk_count;
              current_record.read1.breakpoint.sequence = (&read_seq.clone()[0..20]).to_string();
              // current_record.read1.breakpoint.coordinate = adj_right_pos;
            }
          }
        } else {
          if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
          // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
            current_record.read2.sequence = read_seq.clone();
            current_record.read2.me_read[0] = MERead::loader(&record_line, me_size);
            if mobel_anchor {
              current_record.chranchor = ChrAnchor::Read1;
            }

            // record break point signature
            if
              ( adj_left_pos < 1 ) ||
              ( adj_right_pos > me_size )
            {
              bk_count = 1 + bk_count;
              current_record.read2.breakpoint.sequence = (&read_seq.clone()[0..20]).to_string();
            }
          }
        }
      },

      // secondary alignment
      pf if pf >= 256 => {

        // TODO: probably do not record supplementary alignments
        if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
        // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
          if current_record.read2.sequence == "".to_string() {
            current_record.read1.me_read.push(MERead::loader(&record_line, me_size));
            if mobel_anchor {
              current_record.chranchor = ChrAnchor::Read2;
            }
          } else {
            current_record.read2.me_read.push(MERead::loader(&record_line, me_size));
            if mobel_anchor {
              current_record.chranchor = ChrAnchor::Read1;
            }
          }
        }
      },

      _ => (),
    }

    // reset anchor switch
    mobel_anchor = false;
  }

  println!("Break point count: {}", bk_count);
  Ok(println!("{} {}", "File read: ", &me_bam_file))
}
