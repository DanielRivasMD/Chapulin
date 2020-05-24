
// Chapulin wrapper
use chapulin::{*};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};
use std::env;
use clap::{crate_authors, clap_app};
use config::{Config, File};

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

fn main () -> std::io::Result<()> {

  // read configuration from file
  let matches = clap_app!(Chapilin =>
    (version: "1.0")
    (author: crate_authors!())
    (about: "Mobile Element Identification")
    (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
    // (@arg INPUT: +required "Sets the input file to use")
    // (@arg debug: -d ... "Sets the level of debugging information")
    // (@subcommand test =>
    //   (about: "controls testing features")
    //   (version: "1.3")
    //   (author: "Someone E. <someone_else@other.com>")
    //   (@arg verbose: -v --verbose "Print test information verbosely")
    // )
  )
  .get_matches();

  println!("running {:?}", matches.value_of("CONFIG"));

  let config = matches.value_of("CONFIG").unwrap();
  println!("A config file was passed in: {}", config);

  let mut settings = Config::default();
    settings
      .merge(File::with_name(config)).unwrap();

  // interpret settings into variables
  let settings_hm = settings.try_into::<HashMap<String, String>>().unwrap();

  let directory = settings_hm.get("directory").unwrap();
  let me_library_file = settings_hm.get("mobile_element_library").unwrap();
  let me_align = settings_hm.get("mobile_element_alignment").unwrap();
  let cl_align = settings_hm.get("reference_genome_alignment").unwrap();

  let now = SystemTime::now();

  // initiate HashMap
  let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
  let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));

  // let mut record_collection = HashMap::new();
  // let mut anchor_registry = HashMap::new();

  // TODO: write pre processing recomendations => fastq filtering, alignment

  // mobile elements module
  let c_me_record_collection = mutex_record_collection.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  modules::mobile_elements::me_controller(
    directory,
    me_library_file,
    me_align,
    c_me_record_collection,
  )?;

  match now.elapsed() {
    Ok(elapsed) => {
      println!("{} secs", elapsed.as_secs_f64());
    }

    Err(e) => {
      // an error occurred!
      println!("Error: {:?}", e);
    }
  }

  // modules::mobile_elements::me_controller(
  //   &mut record_collection,
  // )?;

  // chromosomal loci module
  let c_cl_record_collection = mutex_record_collection.clone();
  let c_cl_anchor_registry = mutex_anchor_registry.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  modules::chromosomal_loci::cl_controller(
    directory,
    cl_align,
    c_cl_record_collection,
    c_cl_anchor_registry,
  )?;

  match now.elapsed() {
    Ok(elapsed) => {
      println!("{} secs", elapsed.as_secs_f64());
    }

    Err(e) => {
      // an error occurred!
      println!("Error: {:?}", e);
    }
  }

  // modules::chromosomal_loci::cl_controller(
  //   &mut record_collection,
  //   &mut anchor_registry,
  // )?;

  // peak identification module
  let c_pi_record_collection = mutex_record_collection.clone();
  let c_pi_anchor_registry = mutex_anchor_registry.clone();
  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  modules::peak_identification::pi_controller(
    c_pi_record_collection,
    c_pi_anchor_registry,
  )?;

  match now.elapsed() {
    Ok(elapsed) => {
      println!("{} secs", elapsed.as_secs_f64());
    }

    Err(e) => {
      // an error occurred!
      println!("Error: {:?}", e);
    }
  }

  // modules::peak_identification::pi_controller(
  //   &record_collection,
  //   &anchor_registry,
  // )?;

  // TODO: build interphase to PostgreSQL

  // // output message to log
  // for (key, val) in mutex_record_collection.lock().unwrap().iter() {
  //   println!("key: {}\nval: {:#?}", key, val.chranchor);
  // }

  // println!("{:#?}", mutex_record_collection.lock().unwrap().get("SRR556146.78"));

  println!("Length of Hashmap: {}", mutex_record_collection.lock().unwrap().len());

  match now.elapsed() {
    Ok(elapsed) => {
      println!("{} secs", elapsed.as_secs_f64());
    }

    Err(e) => {
      // an error occurred!
      println!("Error: {:?}", e);
    }
  }
  Ok(())
}
