
use std::collections::HashMap;

// use regex::Regex;

use crate::utils::file_reader;
use crate::utils::read_record::{*};
use crate::utils::me_library::{*};

pub fn me_identificator(
  me_bam_file: &String,
  hm_record_collection: &mut HashMap<String, ReadRecord>,
  hm_me_collection: &HashMap<String, MElibrary>,
) -> std::io::Result<()> {

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&me_bam_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    // load line into vector
    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    let proviral_flag: i32 = record_line[1].parse().unwrap();
    let read_id = record_line[0].to_string();
    let read_position: i32 = record_line[3].parse().unwrap();
    let proviral_id = record_line[2].to_string();

    // retrieve mobile element library records

    // let me_record = match me_val {
    //   Some(y) => y,
    //   None => (),
    // };

    let me_option = hm_me_collection.get(&proviral_id);

    match me_option {

      Some(me_record) => {

        match proviral_flag {

          // primary alignment
          pf if pf <= 255 => {

            // TODO: collect both reads on insert
            // TODO: tag read pairs at break point site
            // TODO: determine breakpoint at upstream & downstream junction

            // TODO: add more filter before loading mobile element aligned read to hashmap. alignment position

            // TODO: define junctions
            // downstream junction
            if me_record.annotations_erv.ltr3 && me_record.me_size > read_position {

            // upstream junction
            } else if me_record.annotations_erv.ltr5 && me_record.me_size < read_position {

            }

            // TODO: temporary
            let me_upstream_limit = me_record.me_size;
            let me_downstream_limit = 200;

            if read_position <= me_upstream_limit || read_position >= me_downstream_limit {
              // if read_position <= me_upstream_limit || read_position >= me_downstream_limit {

              if ! hm_record_collection.contains_key(&read_id) {
                hm_record_collection.insert((&read_id).to_string(), ReadRecord::new());

                if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
                  current_record.read1.sequence = record_line[9].to_string();
                  current_record.read1.me_read[0].mobel = record_line[2].to_string();
                  current_record.read1.me_read[0].flag =  record_line[1].parse().unwrap();
                  current_record.read1.me_read[0].pos =  record_line[3].parse().unwrap();
                  current_record.read1.me_read[0].cigar =  record_line[5].to_string();
                }
              } else {
                if let Some(current_record) = hm_record_collection.get_mut(&read_id) {
                  current_record.read2.sequence = record_line[9].to_string();
                  current_record.read2.me_read[0].mobel = record_line[2].to_string();
                  current_record.read2.me_read[0].flag = record_line[1].parse().unwrap();
                  current_record.read2.me_read[0].pos = record_line[3].parse().unwrap();
                  current_record.read2.me_read[0].cigar = record_line[5].to_string();
                }
              }
            }
          },

          // secondary alignment
          pf if pf >= 256 => {

            // TODO: if secondary hits are recorded, change the loading method as with primary
          }

          _ => (),
        }
      },

      None => (),
    }
  }

  Ok(println!("{} {}", "File read: ", &me_bam_file))
}
