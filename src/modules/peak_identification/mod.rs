////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::thread;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use crate::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// modules
pub mod pi_me_mapping;
mod pi_sv_mapping;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pi_me_controller(
  output: String,
  errata: String,
  chr_registry: alias::RegistryChr,
  me_library: alias::LibraryME,
  dir_registry: alias::RegistryDir,
  me_record: alias::RecordME,
) -> alias::AnyResult {
  let chromosome_vec =
    chr_constructor(chr_registry.clone(), me_library.clone());

  for okey in chromosome_vec {
    let coutput = output.clone();
    let cerrata = errata.clone();
    let cme_record = me_record.clone();
    let cchr_registry = chr_registry.clone();
    let cme_library = me_library.clone();

    let pi_me_handle = thread::spawn(move || {
      pi_me_mapping::pi_me_identifier(
        &okey,
        &coutput,
        &cerrata,
        cchr_registry,
        cme_library,
        cme_record,
      )
      .expect("TODO thread error");
    });
    pi_me_handle.join().expect("MESSAGE_JOIN");
  }
  // TODO: gather all positions & output a comprenhensive list

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn pi_sv_controller(
  _output: String,
  _errata: String,
  chr_registry: alias::RegistryChr,
  me_library: alias::LibraryME,
  sv_record: alias::RecordSV,
) -> alias::AnyResult {
  let chromosome_vec =
    chr_constructor(chr_registry.clone(), me_library.clone());

  for okey in chromosome_vec {
    let cchr_registry = chr_registry.clone();
    let cme_library = me_library.clone();
    let csv_record = sv_record.clone();

    let pi_sv_handle = thread::spawn(move || {
      pi_sv_mapping::pi_sv_identifier(
        &okey,
        cchr_registry,
        cme_library,
        csv_record,
      )
      .expect("TODO thread error");
    });
    pi_sv_handle.join().expect("MESSAGE_JOIN");
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn chr_constructor(
  chr_registry: alias::RegistryChr,
  me_library: alias::LibraryME,
) -> Vec<String> {
  // iterate on reference chromosomes
  let mut chromosome_vec = Vec::new();
  for okey in me_library.lock().unwrap().keys() {
    let ckey = okey.clone();

    if chr_registry.lock().unwrap().contains_key(okey) {
      chromosome_vec.push(ckey);
    }
  }

  chromosome_vec
}

////////////////////////////////////////////////////////////////////////////////////////////////////
