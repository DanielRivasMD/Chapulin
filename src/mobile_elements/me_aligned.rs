
use std::collections::HashMap;

use crate::file_reader;
use regex::Regex;

use super::me_dstruct::*;

pub fn me_identificator(
  me_bam_file: &String
) -> std::io::Result<()> {

  // define regex
  let re = Regex::new(r"^\*").unwrap();

  // initiate HashMap
  let mut primary_me_collection: HashMap<String, PrimaryME> = HashMap::new();
  let mut secondary_me_collection: HashMap<String, SecondaryME> = HashMap::new();

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&me_bam_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    let tmp_pf: i32 = record_line[1].parse().unwrap();
    let tmp_id = record_line[0].to_string();

    match tmp_pf {

        pf if pf <= 255 => {

          if ! primary_me_collection.contains_key(&tmp_id) {

            // println!("found: {} => {}", record_line[0], record_line[1]);

            primary_me_collection.insert((&tmp_id).to_string(), PrimaryME::new());

            if let Some(current_record) = primary_me_collection.get_mut(&tmp_id) {

              current_record.read_id = record_line[0].to_string();
              current_record.r1proviral_flag = record_line[1].parse().unwrap();
              current_record.r1mobel = record_line[2].to_string();
              current_record.r1proviral_pos = record_line[3].parse().unwrap();
              current_record.r1proviral_cigar = record_line[5].to_string();
              current_record.r1read_sequence = record_line[9].to_string();

              // *current_record.r1proviral_flag = record_line[1].parse().unwrap();
              // *current_record = PrimaryME {
              //   read_id: record_line[0].to_string(),
              //   r1proviral_flag: record_line[1].parse().unwrap(),
              //   r1mobel: record_line[2].to_string(),
              //   r1proviral_pos: record_line[3].parse().unwrap(),
              //   r1proviral_cigar: record_line[5].to_string(),
              //   r1read_sequence: record_line[9].to_string(),
              //   r2proviral_flag: 0,
              //   r2mobel: "".to_string(),
              //   r2proviral_pos: 0,
              //   r2proviral_cigar: "".to_string(),
              //   r2read_sequence: "".to_string(),
              // };
            }
            // assert_eq!(map[&1], "b");

            // primary_me_collection.entry((&tmp_id).to_string()).r1proviral_flag = record_line[1].parse().unwrap();

            // primary_me_collection.read_id = record_line[0].to_string();
            // primary_me_collection.r1proviral_flag = record_line[1].parse().unwrap();
            // primary_me_collection.r1mobel = record_line[2].to_string();
            // primary_me_collection.r1proviral_pos = record_line[3].parse().unwrap();
            // primary_me_collection.r1proviral_cigar = record_line[5].to_string();
            // primary_me_collection.r1read_sequence = record_line[9].to_string();


            // let ref_pcol = &primary_me_collection.entry(tmp_id);


            // println!("{:?}", primary_me_collection.entry(tmp_id));


            // let ref_pcol.r2proviral_flag = record_line[1].parse().unwrap();

            // primary_me_collection.entry(tmp_id).r2proviral_flag = record_line[1].parse().unwrap();
            // let primary_me_collection[record_line[0]].r2mobel = record_line[2].to_string();
            // let primary_me_collection[record_line[0]].r2proviral_pos = record_line[3].parse().unwrap();
            // let primary_me_collection[record_line[0]].r2proviral_cigar = record_line[5].to_string();
            // let primary_me_collection[record_line[0]].r2read_sequence = record_line[9].to_string();

          } else {

            if let Some(current_record) = primary_me_collection.get_mut(&tmp_id) {

              current_record.r2proviral_flag = record_line[1].parse().unwrap();
              current_record.r2mobel = record_line[2].to_string();
              current_record.r2proviral_pos = record_line[3].parse().unwrap();
              current_record.r2proviral_cigar = record_line[5].to_string();
              current_record.r2read_sequence = record_line[9].to_string();

            }
            // primary_me_collection.insert(tmp_id, PrimaryME {
              // read_id: record_line[0].to_string(),
              // r1proviral_flag: record_line[1].parse().unwrap(),
              // r1mobel: record_line[2].to_string(),
              // r1proviral_pos: record_line[3].parse().unwrap(),
              // r1proviral_cigar: record_line[5].to_string(),
              // r1read_sequence: record_line[9].to_string(),
              // r2proviral_flag: 0,
              // r2mobel: "".to_string(),
              // r2proviral_pos: 0,
              // r2proviral_cigar: "".to_string(),
              // r2read_sequence: "".to_string(),
            // });
          }
        }

      pf if pf >= 256 => {
        secondary_me_collection.insert(record_line[0].to_string(), SecondaryME {
          read_id: record_line[0].to_string(),
          proviral_flag: record_line[1].parse().unwrap(),
          mobel: record_line[2].to_string(),
          proviral_pos: record_line[3].parse().unwrap(),
          proviral_cigar: record_line[5].to_string(),
        });
      }

      _ => println!("extra")

    }
  }

  for (key, val) in primary_me_collection.iter() {
    // println!("key: {} => {}\nval: {} {}", key, val.r1proviral_flag, val.r1read_sequence, val.reverser());
    println!("key: {} => {}\nval: {}", key, val.r2proviral_flag, val.r2read_sequence);
  }

  // for (key, val) in secondary_me_collection.iter() {
  //   println!("key: {}\nval: {:?}", key, val);
  // }

  // output message to log
  Ok(println!("{} {}", "File read: ", &me_bam_file))
}
