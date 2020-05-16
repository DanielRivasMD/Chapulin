
// standard libraries
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

// crate utilities
use crate::{
  utils::{
    file_reader::file_reader,
    read_record::ReadRecord,
    anchor_read::AnchorRead,
    chranchor_enum::ChrAnchor,
  },
  settings::{
    constants::MAPQ,
  }
};

pub fn cl_mapper(
  cl_bam_file: &String,
  hm_collection: Arc<Mutex<HashMap<String, ReadRecord>>>,
  an_registry: Arc<Mutex<HashMap<String, Vec<String>>>>,
  // chr_max: Arc<Mutex<HashMap<String, i32>>>,
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

        // println!("{:#?}\n{:#?}", current_record, record_line);
        if
          (current_record.read1.sequence == record_line[9].to_string()) ||
          (current_record.read1.sequence_reverser() == record_line[9].to_string())
        {
          current_record.read1.chr_read[0] = AnchorRead::loader(&record_line);

        } else if
          (current_record.read2.sequence == record_line[9].to_string()) ||
          (current_record.read2.sequence_reverser() == record_line[9].to_string())
        {
          current_record.read2.chr_read[0] = AnchorRead::loader(&record_line);
        }

        // println!("{:?}", current_record);
        match current_record.chranchor {
          ChrAnchor::Read1 => {
            if current_record.read1.chr_read[0].mapq < MAPQ && current_record.read1.chr_read[0].chr != "".to_string()
            {
              mapq_switch = true;
            }
          },
          ChrAnchor::Read2 => {
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
          // chr_max.lock().unwrap().insert(record_line[2].to_string(), 0);

          // an_registry.insert(record_line[2].to_string(), Vec::new());
        }



        // if hm_collection.lock().unwrap().get(record_line[0]).unwrap().read1.chr_read[0].pos > *chr_max.lock().unwrap().get_mut(record_line[2]).unwrap() {
        //   chr_max.lock().unwrap().insert(record_line[2].to_string(), hm_collection.lock().unwrap().get(record_line[0]).unwrap().read1.chr_read[0].pos);
        // }

        // let mut tmp_pos = 0;
        //
        // if let Some(tmp_entry) = hm_collection.lock().unwrap().get(record_line[0]) {
        //   tmp_pos = tmp_entry.read1.chr_read[0].pos;
        // }
        //
        // if tmp_pos > chr_max_val {
        //   chr_max_val = tmp_pos;
        // }


        // if let Some(current_chr) = chr_max.lock().unwrap().get(record_line[2]) {
        //   if let Some(tmp_entry) = hm_collection.lock().unwrap().get_mut(record_line[0]) {
        //
        //     let tmp_bool = tmp_entry.read1.chr_read[0].pos > *current_chr;
        //
        //     println!("{} {} {}", current_chr, &tmp_entry.read1.chr_read[0].pos, tmp_bool);
        //
        //     if tmp_bool {
        //       chr_max.lock().unwrap().insert(record_line[2].to_string(), tmp_entry.read1.chr_read[0].pos);
        //       // current_chr = &mut tmp_entry.read1.chr_read[0].pos;
        //     }
        //
        //
        //     // chr_max.lock().unwrap().insert(record_line[2].to_string(), tmp_entry.read1.chr_read[0].pos);
        //   }
        // }

        if let Some(current_chr) = an_registry.lock().unwrap().get_mut(record_line[2]) {
        // if let Some(current_chr) = an_registry.get_mut(record_line[2]) {
          current_chr.push(record_line[0].to_string())


        }
      }
    }
  }

  // println!("{} {}\n", "File read: ", &cl_bam_file);
  Ok(println!("{} {}", "File read: ", &cl_bam_file))
}
