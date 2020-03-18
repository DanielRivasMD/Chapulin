
mod me_dstruct;
mod me_aligned;
// mod me_registry;

pub fn me_controller() -> std::io::Result<()> {

  // shortest way to handle error
  let me_aligned_file = "/Users/drivas/Factorem/Chapulin/test/head.sam".to_string();
  me_aligned::me_identificator(&me_aligned_file).expect(&me_aligned_file);

  Ok(())
}
