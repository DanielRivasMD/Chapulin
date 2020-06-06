
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// crate utilities
use crate::{
  utils::{
    file_reader::file_reader,
    me_chimeric_pair::MEChimericPair,
    chr_anchor::ChrAnchor,
    chr_anchor_enum::ChrAnchorEnum,
  },
  settings::{
    constants::MAPQ,
  }
};


pub fn cl_mapper(
  cl_bam_file: &String,
  hm_collection: Arc<Mutex<HashMap<String, MEChimericPair>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
) -> std::io::Result<()> {

// pub fn cl_mapper(
//   cl_bam_file: &String,
//   hm_collection: &mut HashMap<String, ReadRecord>,
//   an_registry: &mut HashMap<String, Vec<String>>,
// ) -> std::io::Result<()> {

  // load file
  let (mut reader, mut buffer) = file_reader(&cl_bam_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    if hm_collection.lock().unwrap().contains_key(record_line[0]) {
    // if hm_collection.contains_key(record_line[0]) {

      let mut mapq_switch = false;

      if let Some(current_record) = hm_collection.lock().unwrap().get_mut(record_line[0]) {
        // if let Some(current_record) = hm_collection.get_mut(record_line[0]) {

        if
          (current_record.read1.sequence == record_line[9].to_string()) ||
          (current_record.read1.sequence_reverser() == record_line[9].to_string())
        {
          current_record.read1.chr_read[0] = ChrAnchor::loader(&record_line);

        } else if
          (current_record.read2.sequence == record_line[9].to_string()) ||
          (current_record.read2.sequence_reverser() == record_line[9].to_string())
        {
          current_record.read2.chr_read[0] = ChrAnchor::loader(&record_line);
        }

        match current_record.chranch {
          ChrAnchorEnum::Read1 => {
            if current_record.read1.chr_read[0].mapq < MAPQ && current_record.read1.chr_read[0].chr != "".to_string()
            {
              mapq_switch = true;
            }
          },
          ChrAnchorEnum::Read2 => {
            if current_record.read2.chr_read[0].mapq < MAPQ && current_record.read2.chr_read[0].chr != "".to_string()
            {
              mapq_switch = true;
            }
          },
          _ => (),
        };
      }


      if mapq_switch {
        hm_collection.lock().unwrap().remove(record_line[0]);
      } else {
        // register chromosome anchors
        if ! an_registry.lock().unwrap().contains_key(record_line[2]) {
        // if ! an_registry.contains_key(record_line[2]) {
          an_registry.lock().unwrap().insert(record_line[2].to_string(), Vec::new());
          // an_registry.insert(record_line[2].to_string(), Vec::new());
        }

        if let Some(current_chr) = an_registry.lock().unwrap().get_mut(record_line[2]) {
        // if let Some(current_chr) = an_registry.get_mut(record_line[2]) {
          current_chr.push(record_line[0].to_string())


        }
      }
    }
  }

  Ok(println!("{} {}", "File read: ", &cl_bam_file))
}
