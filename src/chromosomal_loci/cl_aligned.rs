
use std::collections::HashMap;

use crate::file_reader;
use regex::Regex;

use super::cl_dstruct::*;

pub fn cl_mapper(
  cl_bam_file: &String
) -> std::io::Result<()> {

  // define regex
  let re = Regex::new(r"^\*").unwrap();

  // initiate HashMap
  let mut cl_collection: HashMap<String, ReadCL> = HashMap::new();

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&cl_bam_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    let record_line: Vec<&str> = line?.trim().split("\t").collect();

    let tmp_cf: i32 = record_line[1].parse().unwrap();
    let tmp_id = record_line[0].to_string();

    // TODO: load chromosome aligned read to hashmap

  }

  Ok(println!("{} {}", "File read: ", &cl_bam_file))
}
