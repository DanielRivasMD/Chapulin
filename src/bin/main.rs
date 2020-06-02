
// Chapulin wrapper
use chapulin::{*};
use clap::{
  clap_app,
  crate_authors,
  crate_version,
};

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
    (version: crate_version!())
    (author: crate_authors!())
    (about: "Mobile Element Identification")

    (@subcommand ME =>
      (version: crate_version!())
      (author: crate_authors!())
      (about: "ME subcommand")
      (@arg verbose: -v --verbose "Print test verbosely")
      (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
    )

    (@subcommand SV =>
      (version: crate_version!())
      (author: crate_authors!())
      (about: "SV subcommand")
      (@arg verbose: -v --verbose "Print test verbosely")
      (@arg CONFIG: -c --config +takes_value "Sets a custom config file")
    )

  )
  .get_matches();

  // ME controller
  if let Some(matches) = matches.subcommand_matches("ME") {
    controllers::me_subcmd::me_subcmd(matches)?;
  }

  // SV controller
  if let Some(matches) = matches.subcommand_matches("SV") {
    controllers::sv_subcmd::sv_subcmd(matches)?;
  }

  Ok(())
}
