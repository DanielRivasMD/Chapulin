
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// crate utilities
use crate::{
  utils::{
    file_reader::file_reader,
    me_chimeric_pair::MEChimericPair,
    me_library::MElibrary,
    me_anchor::MEAnchor,
    cigar::CIGAR,
    chr_anchor_enum::ChrAnchorEnum,
    flag_interpretor::interpretor,
  },
  settings::{
    constants::ME_LIMIT,
  },
};


pub fn me_identificator(
  me_bam_file: &String,
  hm_record_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  hm_me_collection: &HashMap<String, MElibrary>,
) -> std::io::Result<()> {

// pub fn me_identificator(
//   me_bam_file: &String,
//   hm_record_collection: &mut HashMap<String, ReadRecord>,
//   hm_me_collection: &HashMap<String, MElibrary>,
// ) -> std::io::Result<()> {

  // load file
  let (mut reader, mut buffer) = file_reader(&me_bam_file);

  // declare initial values
  let mut prev_read_id = String::new();
  let mut purge_switch = true;
  let mut mobel_anchor = false;
  let mut me_size = 0;
  let mut mobel_orientation = String::new();
  // let mut bk_count = 0;

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    // load line into vector
    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    // update read id
    let read_id = record_line[0].to_string();

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

    // retrieve mobile element library records
    let me_option = hm_me_collection.get(&mobel);
    match me_option {
      Some(me_record) => {
        me_size = me_record.me_size;
      },
      None => (),
    }

    // purge read pairs
    // println!("Post load: {} {} {} {} => {} {}", prev_read_id, read_id, purge_switch, mobel_anchor, record_line[9], pv_flag);
    if ! ( prev_read_id == read_id || prev_read_id == "".to_string() ) {
      // evaluate read batch
      if purge_switch {
        // println!("Deleting: {}", prev_read_id);
        hm_record_collection.lock().unwrap().remove(&prev_read_id);
        // hm_record_collection.remove(&read_id);
      } else {
        // if let Some(tmp_record) = hm_record_collection.lock().unwrap().get(&prev_read_id) {
        //   // println!("{}\t{:?}", prev_read_id, tmp_record.chranchor);
        // }
      }
      // println!();

      // reset purge switch
      purge_switch = true;
    }

    // println!("{:?}", record_line);
    // tagging
    if adj_left_pos <= ME_LIMIT && read_orientation {
      purge_switch = false;
      mobel_anchor = true;
      mobel_orientation = "upstream".to_string();
    } else if me_size - adj_right_pos <= ME_LIMIT && ! read_orientation {
      purge_switch = false;
      mobel_anchor = true;
      mobel_orientation = "downstream".to_string();
    }

      // println!("Delta check: {:?}", hm_record_collection.lock().unwrap().get(&prev_read_id));
      // println!();

    // match on proviral flag
    match pv_flag { // this check is much faster than using binary interpretor

      // primary alignment
      pf if pf <= 255 => {
      // println!();

              // println!("Flag testing: {:?} {}", hm_record_collection.lock().unwrap().contains_key(&read_id), read_id);

        if ! hm_record_collection.lock().unwrap().contains_key(&read_id) {
        // if ! hm_record_collection.contains_key(&read_id) {
          hm_record_collection.lock().unwrap().insert((&read_id).to_string(), MEChimericPair::new());
          // hm_record_collection.insert((&read_id).to_string(), ReadRecord::new());

          // println!("Loading 1... {} {} {}", read_id, mobel_anchor, purge_switch);
      // println!();
          if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
          // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
            current_record.read1.sequence = read_seq.clone();
            current_record.read1.me_read[0] = MEAnchor::loader(&record_line, me_size, &mobel_orientation);
            if mobel_anchor { current_record.chranch = ChrAnchorEnum::Read2; }

            // // record break point signature
            // if
            //   ( adj_left_pos < 1 ) ||
            //   ( adj_right_pos > me_size )
            // {
            //   // TODO: breakpoint
            //   bk_count = 1 + bk_count;
            //   current_record.read1.breakpoint.sequence = (&read_seq.clone()[0..20]).to_string();
            //   // current_record.read1.breakpoint.coordinate = adj_right_pos;
            // }
          }
        } else {
          // println!("Loading 2... {} {} {}", read_id, mobel_anchor, purge_switch);
      // println!();
          if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
          // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
            current_record.read2.sequence = read_seq.clone();
            current_record.read2.me_read[0] = MEAnchor::loader(&record_line, me_size, &mobel_orientation);
            if mobel_anchor { current_record.chranch = ChrAnchorEnum::Read1; }

            // // record break point signature
            // if
            //   ( adj_left_pos < 1 ) ||
            //   ( adj_right_pos > me_size )
            // {
            //   bk_count = 1 + bk_count;
            //   current_record.read2.breakpoint.sequence = (&read_seq.clone()[0..20]).to_string();
            // }
          }
        }
              // println!("Load check: {:?}", hm_record_collection.lock().unwrap().get(&read_id));
      // println!();
      },

      // secondary alignment
      pf if pf >= 256 => {

        if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
        // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
          if current_record.read2.sequence == "".to_string() {
                    // println!("Supplem 1... {} {} {}", read_id, mobel_anchor, purge_switch);
            current_record.read1.me_read.push(MEAnchor::loader(&record_line, me_size, &mobel_orientation));
            if mobel_anchor { current_record.chranch = ChrAnchorEnum::Read2; }
          } else {
                    // println!("Supplem 2... {} {} {}", read_id, mobel_anchor, purge_switch);
            current_record.read2.me_read.push(MEAnchor::loader(&record_line, me_size, &mobel_orientation));
            if mobel_anchor { current_record.chranch = ChrAnchorEnum::Read1; }
          }
        }
      },

      _ => (),
    }



    // reset anchor switch
    mobel_anchor = false;

    prev_read_id = read_id;

      // if let Some(tmp_record) = hm_record_collection.lock().unwrap().get(&prev_read_id) {
      //   // println!("{}\t{:?}", prev_read_id, tmp_record.chranchor);
      //   println!("{:#?}", tmp_record.chranchor);
      // }

  }

  // evaluate at end of file
  if purge_switch {
          // println!("Last check: {:?}", hm_record_collection.lock().unwrap().get(&prev_read_id));

    hm_record_collection.lock().unwrap().remove(&prev_read_id);
    // hm_record_collection.remove(&read_id);
  }

  // println!("Break point count: {}", bk_count);
  Ok(println!("{} {}", "File read: ", &me_bam_file))
}
