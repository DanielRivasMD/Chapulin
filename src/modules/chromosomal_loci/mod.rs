
mod cl_dstruct;
mod cl_aligned;

pub fn cl_controller() -> std::io::Result<()> {

  // load reference chromosome aligned reads
  let cl_aligned_file = "".to_string();
  cl_aligned::cl_mapper(&cl_aligned_file).expect(&cl_aligned_file);

  Ok(())
}
