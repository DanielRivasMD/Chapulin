
// standard libraries
use std::collections::HashMap;

// crate utilities
use crate::utils::{
  file_reader,
  read_record::{
    ReadRecord,
    // AnchorRead,
  }
};

pub fn cl_mapper(
  cl_bam_file: &String,
  hm_collection: &mut HashMap<String, ReadRecord>,
  an_registry: &mut HashMap<String, Vec<String>>,
) -> std::io::Result<()> {

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&cl_bam_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    let tmp_id: String = record_line[0].to_string();
    let tmp_seq: String = record_line[9].to_string();

        // TODO: define which read in the pair is the anchor to register
        // register chromosome anchors
        if ! an_registry.contains_key(record_line[2]) {
          an_registry.insert(record_line[2].to_string(), Vec::new());
        }

        if let Some(current_chr) = an_registry.get_mut(record_line[2]) {
          current_chr.push(record_line[0].to_string())
        }

        if let Some(current_record) = hm_collection.get_mut(&tmp_id) {

          if current_record.read1.sequence == tmp_seq || current_record.read1.sequence_reverser() == tmp_seq {
            current_record.read1.chr_read[0].chr = record_line[2].to_string();
            current_record.read1.chr_read[0].flag = record_line[1].parse().unwrap();
            current_record.read1.chr_read[0].pos = record_line[3].parse().unwrap();
            current_record.read1.chr_read[0].cigar = record_line[5].to_string();
            current_record.read1.chr_read[0].mapq = record_line[4].to_string();
            current_record.read1.test_seq = record_line[9].to_string();
          } else if current_record.read2.sequence == tmp_seq || current_record.read2.sequence_reverser() == tmp_seq {
            current_record.read2.chr_read[0].chr = record_line[2].to_string();
            current_record.read2.chr_read[0].flag = record_line[1].parse().unwrap();
            current_record.read2.chr_read[0].pos = record_line[3].parse().unwrap();
            current_record.read2.chr_read[0].cigar = record_line[5].to_string();
            current_record.read2.chr_read[0].mapq = record_line[4].to_string();
            current_record.read2.test_seq = record_line[9].to_string();
          } else {
            current_record.debug_seq = record_line[9].to_string();
          }
        }
      }
    }

  Ok(println!("{} {}", "File read: ", &cl_bam_file))
}
