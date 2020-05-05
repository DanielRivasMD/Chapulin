
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

  // evaluate read batch
  let mut read_id = String::new();
  let mut purge_switch = true;

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    // load line into vector
    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    // purge read pairs
    if ! ( read_id == record_line[0].to_string() || read_id == "".to_string() ) {

      if purge_switch {
        hm_record_collection.lock().unwrap().remove(&read_id);
        // hm_record_collection.remove(&read_id);
      }

      // reset purge switch
      purge_switch = true;
    }

    // update read id
    read_id = record_line[0].to_string();

    // retrieve mobile element library records
    let me_option = hm_me_collection.get(&record_line[2].to_string());

    let tmp_flag = record_line[1].parse::<i32>().unwrap();

    match me_option {

      Some(me_record) => {

        // TODO: describe break point signature
        // TODO: define filters for keeping based on breakpoint estimation & read orientation
        // read pair selection criteria

        // println!("flag: {} => {}", record_line[1], interpretor(record_line[1].parse().unwrap(), 5));
        //
        // let cgr = CIGAR::loader(&record_line[5].to_string());
        // println!("position: {} => adjusted: {} => by function: {}", record_line[3], record_line[3].parse::<i32>().unwrap() - cgr.lclip, cgr.left_boundry(record_line[3].parse::<i32>().unwrap()));
        // println!("position: {} => adjusted: {} => by function: {}", record_line[3], record_line[3].parse::<i32>().unwrap() - cgr.lclip + 100, cgr.right_boundry(record_line[3].parse::<i32>().unwrap()));
        // println!("cigar: {} => {:#?}", record_line[5].to_string(), cgr);

        // // let tmp_flag = flag_int(record_line[1].parse().unwrap());
        // let tmp_flag = format!("{:b}", record_line[1].parse::<i32>().unwrap());
        // let mut stat_string = ['0'; 12];
        // println!("{:#?}", stat_string);
        //
        // for i in tmp_flag.char_indices() {
        //   stat_string[i.0] = i.1;
        //   // println!("{:?}", i);
        // }
        //
        // println!("{:#?}", stat_string);
        // println!("{:#?}", tmp_flag);

        let tmp_pos = record_line[3].parse::<i32>().unwrap();
        let tmp_cigar = CIGAR::loader(&record_line[5].to_string());

        if (
            tmp_cigar.left_boundry(tmp_pos) <= ME_LIMIT &&
            interpretor(tmp_flag, 5) // upstream
          ) || (
            tmp_cigar.right_boundry(tmp_pos) <= me_record.me_size - ME_LIMIT &&
            ! interpretor(tmp_flag, 5) // downstream
          )

        // ( record_line[3].parse::<i32>().unwrap() <= ME_LIMIT ) ||
            // downstream && record_line[1].parse::<i32>().unwrap() == 0
          // record_line[3].parse::<i32>().unwrap() >= (me_record.me_size - ME_LIMIT) // upstream
        {

          // tag for keeping
          purge_switch = false;
        }
      },

      None => (),
    }

    // match on proviral flag
    match tmp_flag {

      // primary alignment
      pf if pf <= 255 => {

        // let tmpx = format!("{:b}", record_line[1].parse::<i32>().unwrap()).parse::<i32>().unwrap();
        // println!("{}", tmpx);

        if ! hm_record_collection.lock().unwrap().contains_key(&read_id) {
        // if ! hm_record_collection.contains_key(&read_id) {
          hm_record_collection.lock().unwrap().insert((&read_id).to_string(), ReadRecord::new());
          // hm_record_collection.insert((&read_id).to_string(), ReadRecord::new());

          if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
          // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
            current_record.read1.sequence = record_line[9].to_string();
            current_record.read1.me_read[0].mobel = record_line[2].to_string();
            current_record.read1.me_read[0].flag =  record_line[1].parse().unwrap();
            current_record.read1.me_read[0].pos =  record_line[3].parse().unwrap();
            current_record.read1.me_read[0].cigar =  record_line[5].to_string();
          }
        } else {
          if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
          // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
            current_record.read2.sequence = record_line[9].to_string();
            current_record.read2.me_read[0].mobel = record_line[2].to_string();
            current_record.read2.me_read[0].flag = record_line[1].parse().unwrap();
            current_record.read2.me_read[0].pos = record_line[3].parse().unwrap();
            current_record.read2.me_read[0].cigar = record_line[5].to_string();
          }
        }
      },

      // secondary alignment
      pf if pf >= 256 => {

        // TODO: probably do not record supplementary alignments
        if let Some(current_record) = hm_record_collection.lock().unwrap().get_mut(&read_id) {
        // if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
          if current_record.read2.sequence == "".to_string() {
            current_record.read1.me_read.push(MERead {
              mobel: record_line[2].to_string(),
              flag: record_line[1].parse().unwrap(),
              pos: record_line[3].parse().unwrap(),
              cigar: record_line[5].to_string(),
            })
          } else {
            current_record.read2.me_read.push(MERead {
              mobel: record_line[2].to_string(),
              flag: record_line[1].parse().unwrap(),
              pos: record_line[3].parse().unwrap(),
              cigar: record_line[5].to_string(),
            })
          }
        }
      },

      _ => (),
    }
  }

  Ok(println!("{} {}", "File read: ", &me_bam_file))
}
