
use std::collections::HashMap;

use regex::Regex;

use crate::utils::file_reader;
use crate::utils::record::{*};

pub fn me_identificator(
  me_bam_file: &String,
  hm_record_collection: &mut HashMap<String, ReadRecord>,
  hm_me_collection: &HashMap<String, MElibrary>,
) -> std::io::Result<()> {

  // define regex
  let _re = Regex::new(r"^\*").unwrap();

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&me_bam_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    let proviral_flag: i32 = record_line[1].parse().unwrap();
    let read_id = record_line[0].to_string();
    let read_position: i32 = record_line[3].parse().unwrap();
    let proviral_id = record_line[2].to_string();

    // purgr incomplete reads
    // TODO: if length is not as expect & is not "*" abbreviated

    match tmp_pf {

      pf if pf <= 255 => {

        // TODO: collect both reads on insert
        // TODO: tag read pairs at break point site
        // TODO: determine breakpoint at upstream & downstream junction

        // TODO: add more filter before loading mobile element aligned read to hashmap. alignment position

        if tmp_pos <= me_upstream_limit || tmp_pos >= me_downstream_limit {

          if ! hm_record_collection.contains_key(&read_id) {

            hm_record_collection.insert((&read_id).to_string(), ReadRecord::new());

            if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
              current_record.read1.mobel = record_line[2].to_string();
              current_record.read1.pv_flag = record_line[1].parse().unwrap();
              current_record.read1.pv_pos = record_line[3].parse().unwrap();
              current_record.read1.pv_cigar = record_line[5].to_string();
              current_record.read1.sequence = record_line[9].to_string();
            }

          } else {

            if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
              current_record.read2.mobel = record_line[2].to_string();
              current_record.read2.pv_flag = record_line[1].parse().unwrap();
              current_record.read2.pv_pos = record_line[3].parse().unwrap();
              current_record.read2.pv_cigar = record_line[5].to_string();
              current_record.read2.sequence = record_line[9].to_string();
            }
          }
        }
      }

      pf if pf >= 256 => {

        // TODO: if secondary hits are recorded, change the loading method as with primary
        // secondary_me_collection.insert(record_line[0].to_string(), SecondaryME {
        //   read_id: record_line[0].to_string(),
        //   proviral_flag: record_line[1].parse().unwrap(),
        //   mobel: record_line[2].to_string(),
        //   proviral_pos: record_line[3].parse().unwrap(),
        //   proviral_cigar: record_line[5].to_string(),
        // });

      }

      _ => println!("extra")

    }

  }

  Ok(println!("{} {}", "File read: ", &me_bam_file))
}
