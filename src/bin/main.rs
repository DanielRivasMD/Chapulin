
// Chapulin wrapper
use chapulin::{*};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime};
use std::env;
use clap::{App, Arg};
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

fn main() -> std::io::Result<()> {

  // read configuration
  let matches = App::new("Chapiln")
  .arg(
  Arg::new("config")
      .about("sets the config file to use")
      .takes_value(true)
      .short('c')
      .long("config"),
  )
  .get_matches();

  if let Some(config) = matches.value_of("config") {
    println!("A config file was passed in: {}", config);

    let mut settings = Config::default();
      settings
        // File::with_name(..) is shorthand for File::from(Path::new(..))
        .merge(File::with_name(config)).unwrap();
        // .merge(File::with_name("conf/00-default.toml")).unwrap()
        // .merge(File::from(Path::new("conf/05-some.yml"))).unwrap()
        // .merge(File::from(Path::new("conf/99-extra.json"))).unwrap();

      // Print out our settings (as a HashMap)

        // println!("\n{:?} \n\n-----------",
        //        settings.try_into::<HashMap<String, String>>().unwrap());

    let hm = settings.try_into::<HashMap<String, String>>().unwrap();

    if hm.contains_key("x") {

      let xv = hm.get("x").unwrap();
      println!("values of xv is {:?}", xv);
      // println!("values of x is {:?}", hm.get("x").unwrap());
    }

  }

  let args: Vec<String> = env::args().collect();

  // let mut settings = config::Config::default();
  // settings
  //   // Add in `./Settings.toml`
  //   .merge(config::File::with_name(&*args[1].to_string())).unwrap()
  //   // Add in settings from the environment (with a prefix of APP)
  //   // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
  //   .merge(config::Environment::with_prefix("APP")).unwrap();
  //
  // // Print out our settings (as a HashMap)
  // println!("{:?}",
  //   settings.try_into::<HashMap<String, String>>().unwrap());
  //

  // config::read_settings::read_config(
  //   &args[1],
  // );


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
    &args[1],
    &args[2],
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
    &args[3],
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
