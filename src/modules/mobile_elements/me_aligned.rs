
// standard libraries
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex
};

// crate utilities
use crate::utils::{
  file_reader,
  read_record::ReadRecord,
  me_library::MElibrary,
  me_read::MERead,
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

    // TODO: define "me_limit" as a constant in configuration
    let me_limit = 200;

    match me_option {

      Some(me_record) => {

        // TODO: describe break point signature
        // TODO: define filters for keeping based on breakpoint estimation & read orientation
        // read pair selection criteria
        if
          record_line[3].parse::<i32>().unwrap() <= me_limit || // downstream
          record_line[3].parse::<i32>().unwrap() >= (me_record.me_size - me_limit) // upstream
        {

          // tag for keeping
          purge_switch = false;
        }
      },

      None => (),
    }

    // match on proviral flag
    match record_line[1].parse::<i32>().unwrap() {

      // primary alignment
      pf if pf <= 255 => {

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
