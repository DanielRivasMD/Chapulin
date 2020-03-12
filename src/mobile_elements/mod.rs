
mod me_dstruct;
mod me_aligned;
// mod me_registry;

pub fn me_controller() -> std::io::Result<()> {

  // let database_file = "/Users/drivas/Factorem/Chapulin/test/mobile_element_database".to_string();
  // me_registry::me_registator(&database_file).expect(&database_file);

  // shortest way to handle error
  let me_aligned_file = "/Users/drivas/Factorem/Chapulin/test/head.sam".to_string();
  me_aligned::me_identificator(&me_aligned_file).expect(&me_aligned_file);

  // let me_align_err = me_aligned::me_identificator(&first_file);
  // me_align_err.expect(&first_file);

  // let me_aligned_check = me_aligned::me_identificator(&first_file);
  // if let Err(e) = me_aligned_check {
  //   println!("{}: {}", e, &first_file)
  // }

  // let me_aligned_check = me_aligned::me_identificator(&first_file);
  // match me_aligned_check {
  //   Ok(_) => { println!("{}", "nice!") }
  //   Err(e) => { println!("{}: {}", e, &first_file) }
  // }

  // let

  Ok(())
}
