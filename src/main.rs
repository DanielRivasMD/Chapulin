#![allow(dead_code)]

mod file_reader;
mod mobile_elements;
mod chromosomal_loci;
mod peak_identification;

fn main() -> std::io::Result<()> {

  // TODO: write pre processing recomendations => fastq filtering, alignment

  // mobile elements module
  mobile_elements::me_controller()?;

  // chromosomal loci module
  chromosomal_loci::cl_contorller();

  // peak identification module
  peak_identification::pi_controller();

  // TODO: build interphase to PostgreSQL

  Ok(())
}
