
use chapulin::{*};

/*
the general idea is to create a modulerize, fast & reliable tool for mobile element identification in re sequence projects

hold all configuration variables in one file
read bam files, one from mobile element alignment & two from chromosomal reference alingment, once from disk
load all neccesary data into memory (hashmap) for efficiency. to optimize this process, use two methods:
  1) prefilter read to load, therefore minimizing size of hashmap to hold
  2) load all neccesary data into each struct record and use traits to hold virtual information

collect both, mobile element & chromosomal reference, versions of insert pairs
filter according to quality criteria
perform peak detection & calculate false discovery rate
label chimeric reads for massive break point reconstructions
generate stats at every step
create a safe escape in case of memory failures
create unit tests
*/

fn main() -> std::io::Result<()> {

  // initiate HashMap
  let mut record_collection = utils::hashmap_init::hashmap_init();

  // TODO: write pre processing recomendations => fastq filtering, alignment

  // mobile elements module
  modules::mobile_elements::me_controller(&mut record_collection)?;

  // chromosomal loci module
  modules::chromosomal_loci::cl_controller(&mut record_collection)?;

  // // peak identification module
  // peak_identification::pi_controller();

  // TODO: build interphase to PostgreSQL

  // output message to log
  for (key, val) in record_collection.iter() {
    // println!("key: {} => {}\nval: {} {}", key, val.r1proviral_flag, val.r1read_sequence, val.reverser());
    // println!("key: {} => {}\nval: {}", key, val.r2proviral_flag, val.r2read_sequence);
    println!("key: {}\nval: {:#?}", key, val);
  }

  // for (key, val) in secondary_me_collection.iter() {
  //   println!("key: {}\nval: {:?}", key, val);
  // }


  with_love();
  Ok(())
}
