
use crate::file_reader;
use regex::Regex;

// use super::me_struct;
// pub mod me_struct;
// #[path = "mobile_elements/me_struct.rs"] mod me_struct;

use super::me_dstruct::*;
// use crate::mobile_elements::me_struct::MobileElement;

pub fn me_identificator(
  me_bam_file: &String
) -> std::io::Result<()> {

  // // define regex
  // let re = Regex::new(r"^\*").unwrap();

  // load file
  let (mut reader, mut buffer) = file_reader::file_reader(&me_bam_file);

  // iterate through file
  while let Some(line) = reader.read_line(&mut buffer) {

    // println!("{}", line?.trim());

    let vec: Vec<&str> = line?.trim().split("\t").collect();

    // let seq = vec[9];
    //
    // println!("{}", seq);

    // IDEA: load into struct with sec entries


    let proviral_flag: i32 = vec[1].parse().unwrap();

    if proviral_flag < 2048 {

      match proviral_flag {

        pf if pf <= 255 => {
          let primary_record = PrimaryME {
            read_id: vec[0].to_string(),
            proviral_flag: vec[1].parse().unwrap(),
            mobel: vec[2].to_string(),
            proviral_pos: vec[3].parse().unwrap(),
            proviral_cigar: vec[5].to_string(),
            read_sequence: vec[9].to_string(),
          };
          println!("{:?}", primary_record)
        }
        pf if pf >= 256 => {
          let secondary_record = SecondaryME {
            read_id: vec[0].to_string(),
            proviral_flag: vec[1].parse().unwrap(),
            mobel: vec[2].to_string(),
            proviral_pos: vec[3].parse().unwrap(),
            proviral_cigar: vec[5].to_string(),
          };
          println!("{:?}", secondary_record)
        }
        _ => println!("record out of range")
      }


    //       match age {
    //   21 => println!("age is 21"),                                       // option
    //   22 => println!("age is 22"),                                       // option
    //   23 | 24 => println!("age is 23 or 24"),                            // OR
    //   25..=28 => println!("age is between 25 and 28"),                   // range
    //   n if n < 5 => println!("age is less than 5"),                      // less than
    //   n if n > 50 => println!("age is greater than 50"),                 // more than
    //   _ => println!("age is something else")                             // exhaustive
    // }

    // if proviral_flag < 255 {
    //
    //   let this_record = PrimaryME {
    //     read_id: vec[0].to_string(),
    //     proviral_flag: vec[1].parse().unwrap(),
    //     mobel: vec[2].to_string(),
    //     proviral_pos: vec[3].parse().unwrap(),
    //     proviral_cigar: vec[5].to_string(),
    //     read_sequence: vec[9].to_string(),
    //   };
    // println!("{} {} {}", this_record.read_id, this_record.read_sequence, this_record.reverser());
    // } else {
    //
    // }

      // println!("{}", proviral_flag);
    }
    // let proviral_flag = vec[1].parse();
    // let proviral_flag: i32 = proviral_flag.unwrap();

    // if proviral_flag < 2048 {
    //
    //   // let sequence = vec[9];
    //   let sequence: &str;
    //
    //   if ! re.is_match(&vec[9]) {
    //
    //     sequence = &vec[9];
    //
    //
    //     println!("{}", sequence);
    //
    //   } else {
    //     // let back_seq = sequence.clone;
    //
    //   }


      // println!("{}", sequence);

      // let mut x: () = vec[9];
      //
      // println!("{}", re.is_match(&vec[9]));
      //
      // let x = re.is_match(&vec[9]);
      //
      // if x {
      //   println!("true that")
      // }
      // assert!(re.is_match(&vec[9]));

    // }


    // if( $10 !~/\*/ )
		// 	{
		// 		seq=$10;
		// 	}else{
		// 		seq=seq;
		// 	}
		// 	if( length(seq) != read_length )
		// 	{
		// 		next;
		// 	}
		// 	read_id = $1;
		// 	proviral_flag = $2;
		// 	erv = $3;
		// 	proviral_pos = $4;
		// 	proviral_cigar = $6;
		// 	to_reverse = seq;
		// 	gsub("A", "t", to_reverse) gsub("T", "a", to_reverse) gsub("G", "c", to_reverse) gsub("C", "g", to_reverse) gsub("t", "T", to_reverse) gsub("a", "A", to_reverse) gsub("g", "G", to_reverse) gsub("c", "C", to_reverse);


    // match proviral_flag(n) {
    //   Ok(n) => println!("{}", n),
    //   Err(e) => println!("{}", "didn't make it"),
    // }

    // let ltr = vec[0];
    // let ltr = vec[1];

    // LTR = $1;
		// ERV_id = $2;
		// ERV_size = $3;
		// if ( LTR == "ltr5" )
		// {
		// 	ltr5[ERV_id]=ERV_size;
		// }
		// if ( LTR == "ltr3" )
		// {
		// 	ltr3[ERV_id]=ERV_size;
		// }

  }

  // output message to log
  // TODO: send this to log
  Ok(println!("{} {}", "File read: ", &me_bam_file))
}




// trait ReadSequence {
//   fn reverser(&self) -> String {
//     self.chars().rev().collect()
//   }
// }
//
//
// impl ReadSequence for PrimaryME.read_sequence {}


// // trait ReadSequence {
// pub fn reverser(to_rev_seq: &String) -> String {
//     to_rev_seq.chars().rev().collect()
//   }
// // }
