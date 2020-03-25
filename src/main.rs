#![allow(dead_code)]

mod file_reader;
mod mobile_elements;
mod chromosomal_loci;
mod peak_identification;

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

  // TODO: write pre processing recomendations => fastq filtering, alignment

  // TODO: think about a way to make module communicate

  // mobile elements module
  mobile_elements::me_controller()?;

  // TODO: combine output from mobile elements. probably passing reference to hashmap as argument

  // // chromosomal loci module
  // chromosomal_loci::cl_controller();
  //
  // // peak identification module
  // peak_identification::pi_controller();

  // TODO: build interphase to PostgreSQL

  Ok(())
}
