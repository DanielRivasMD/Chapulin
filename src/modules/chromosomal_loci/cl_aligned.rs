
use std::collections::HashMap;

use crate::utils::file_reader;
use crate::utils::record::{*};

pub fn cl_mapper(
  cl_bam_file: &String,
  hm_collection: &mut HashMap<String, ReadRecord>,
) -> std::io::Result<()> {

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&cl_bam_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    let tmp_id: String = record_line[0].to_string();
    let tmp_seq: String = record_line[9].to_string();

      if hm_collection.contains_key(&tmp_id) {

        if let Some(current_record) = hm_collection.get_mut(&tmp_id) {

          if current_record.read1.sequence == tmp_seq || current_record.read1.sequence_reverser() == tmp_seq {
            current_record.read1.chr = record_line[2].to_string();
            current_record.read1.cl_flag = record_line[1].parse().unwrap();
            current_record.read1.cl_pos = record_line[3].parse().unwrap();
            current_record.read1.cl_cigar = record_line[5].to_string();
            current_record.read1.test_seq = record_line[9].to_string();
            current_record.read1.cl_mapq = record_line[4].to_string();
          } else if current_record.read2.sequence == tmp_seq || current_record.read2.sequence_reverser() == tmp_seq {
            current_record.read2.chr = record_line[2].to_string();
            current_record.read2.cl_flag = record_line[1].parse().unwrap();
            current_record.read2.cl_pos = record_line[3].parse().unwrap();
            current_record.read2.cl_cigar = record_line[5].to_string();
            current_record.read2.test_seq = record_line[9].to_string();
            current_record.read2.cl_mapq = record_line[4].to_string();
          } else {
            current_record.debug_seq = record_line[9].to_string();
          }
        }
      }
    }

  Ok(println!("{} {}", "File read: ", &cl_bam_file))
}
