
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

    if tmp_pf < 2048 {
      match tmp_pf {
        pf if pf <= 255 => {
          primary_me_collection.insert(record_line[0].to_string(), PrimaryME {
            read_id: record_line[0].to_string(),
            proviral_flag: record_line[1].parse().unwrap(),
            mobel: record_line[2].to_string(),
            proviral_pos: record_line[3].parse().unwrap(),
            proviral_cigar: record_line[5].to_string(),
            read_sequence: record_line[9].to_string(),
          });
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
        _ => println!("record out of range")
      }
    }
  }

  for (key, val) in primary_me_collection.iter() {
    println!("key: {}\nval: {} {}", key, val.read_sequence, val.reverser());
  }

  for (key, val) in secondary_me_collection.iter() {
    println!("key: {}\nval: {:?}", key, val);
  }

  // output message to log
  Ok(println!("{} {}", "File read: ", &me_bam_file))
}
