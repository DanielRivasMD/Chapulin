
// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};
use clap::{ArgMatches};
use config::{Config, File};
use anyhow::{Context};
use anyhow::Result as anyResult;

// modules
use crate::modules;

use crate::error::config_error::ChapulinConfigError;


pub fn me_subcmd(
  matches: &ArgMatches
) -> anyResult<()> {

  let mut verbose = false;
  if matches.is_present("verbose") {
    verbose = true;
  }

  let now = SystemTime::now();

  let config = matches.value_of("CONFIG")
    .context(ChapulinConfigError::EmptyConfigOption)?;

  let mut settings = Config::default();
    settings
      .merge(File::with_name(config))
      .context(ChapulinConfigError::NoConfigFile)?;

  // interpret settings into variables
  let settings_hm = settings.try_into::<HashMap<String, String>>()
    .context(ChapulinConfigError::ConfigHashMap)?;

  let directory = settings_hm.get("directory")
    .context(ChapulinConfigError::BadDirectoryVar)?;
  let reference_file = settings_hm.get("reference")
    .context(ChapulinConfigError::BadReferenceVar)?;
  let me_library_file = settings_hm.get("mobile_element_library")
    .context(ChapulinConfigError::BadMELibVar)?;
  let me_align = settings_hm.get("mobile_element_alignment")
    .context(ChapulinConfigError::BadMEAlignVar)?;
  let cl_align = settings_hm.get("reference_genome_alignment")
    .context(ChapulinConfigError::BadReferenceGenomeVar)?;

  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));
  let mutex_chr_assembly = Arc::new(Mutex::new(HashMap::new()));

  // TODO: write pre processing recomendations => fastq filtering, alignment

  // reference genome module
  let c_rg_chr_assembly = mutex_chr_assembly.clone();

  if verbose {
    println!("Running Reference Genome module...");
    println!("Reference file read: {}", reference_file);
  }

  modules::reference_genome::ref_controller(
    directory,
    reference_file,
    c_rg_chr_assembly,
  )?;

  println!("{:?}", now.elapsed().unwrap());

  // mobile elements module
  let c_me_record_collection = mutex_record_collection.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  if verbose {
    println!("Running Mobile Element module...");
    println!("ME alignment file read: {}", me_align);
  }

  modules::mobile_elements::me_controller(
    directory,
    me_library_file,
    me_align,
    c_me_record_collection,
  )?;

  println!("{:?}", now.elapsed().unwrap());

  // chromosomal loci module
  let c_cl_record_collection = mutex_record_collection.clone();
  let c_cl_anchor_registry = mutex_anchor_registry.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  if verbose {
    println!("Running Chromosomal Loci module...");
    println!("Chromosomal alignment file read: {}", cl_align);
  }

  modules::chromosomal_loci::cl_controller(
    directory,
    cl_align,
    c_cl_record_collection,
    c_cl_anchor_registry,
  )?;

  println!("{:?}", now.elapsed().unwrap());

  // peak identification module
  let c_pi_record_collection = mutex_record_collection.clone();
  let c_pi_anchor_registry = mutex_anchor_registry.clone();
  let c_pi_chr_assembly = mutex_chr_assembly.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  if verbose {
    println!("Running Peak Identification module...");
  }

  modules::peak_identification::pi_controller(
    c_pi_record_collection,
    c_pi_anchor_registry,
    c_pi_chr_assembly,
  )?;

  println!("{:?}", now.elapsed().unwrap());

  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  // TODO: build interphase to PostgreSQL

  // // output message to log
  // for (key, val) in mutex_record_collection.lock().unwrap().iter() {
  //   println!("key: {}\nval: {:#?}", key, val.chranchor);
  // }

  // println!("{:#?}", mutex_record_collection.lock().unwrap().get_key_value("SRR556146.443333"));

  Ok(())
}
