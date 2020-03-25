
mod me_dstruct;
mod me_aligned;
mod me_registry;

pub fn me_controller() -> std::io::Result<()> {

  // // load mobile element library
  // let me_library_file = "".to_string();
  // me_registry::me_lib_loader(&me_library_file).expect(&me_library_file);

  // load mobile element aligned reads
  let me_aligned_file = "/Users/drivas/Factorem/Chapulin/test/head.sam".to_string();
  me_aligned::me_identificator(&me_aligned_file).expect(&me_aligned_file);

  Ok(())
}
