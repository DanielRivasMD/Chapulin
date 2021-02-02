////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::{App, Arg, crate_authors, crate_version};

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cli_chapulin() -> App<'static> {

  App::new("chapulin")
    .version(crate_version!())
    .author(crate_authors!())
    .about("
      \nChapulin: Mobile Element Identification
      \nSoftware for mobile element identification in resequenced short-read data with a reference genome.
      \n\n\tAvailable subcommands are:
      \nMobile Element (ME): performs sequence similarity search to a customized mobile element library and insertion calls by probability or a set threshold.
      \nStructural Variant (SV): performs read selection based on alignment data and variant calls by probability or a set threshold.
      \nCache Registering (CR): checks for reference genome and mobile element library cache in configuration directory. In case cahces are not found, reads files and writes cache.
    ")
    .arg(
      Arg::new("verbose")
      .short('v')
      .long("verbose")
      .about("Prints verbosely")
    )

    .subcommand(App::new("ME")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Mobile Element Identification")
      .arg(
        Arg::new("verbose")
        .short('v')
        .long("verbose")
        .about("Prints verbosely")
      )
      .arg(
        Arg::new("logging")
        .short('l')
        .long("log")
        .about("Prints log")
        .takes_value(true)
      )
      .arg(
        Arg::new("config")
        .short('c')
        .long("config")
        .about("Selects config file")
        .takes_value(true)
      )
      .arg(
        Arg::new("chralign")
        .short('a')
        .long("alignment")
        .about("Selects chromosomal alignment")
        .required(true)
        .takes_value(true)
        .default_value("paired")
        .possible_values(&["single", "paired"])
      )
    )

    .subcommand(App::new("SV")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Structural Variant Identification")
      .arg(
        Arg::new("verbose")
        .short('v')
        .long("verbose")
        .about("Prints verbosely")
      )
      .arg(
        Arg::new("logging")
        .short('l')
        .long("log")
        .about("Prints log")
        .takes_value(true)
      )
      .arg(
        Arg::new("config")
        .short('c')
        .long("config")
        .about("Selects config file")
        .takes_value(true)
      )
    )

    .subcommand(App::new("CR")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Cache Registering")
      .arg(
        Arg::new("verbose")
        .short('v')
        .long("verbose")
        .about("Prints verbosely")
      )
      .arg(
        Arg::new("logging")
        .short('l')
        .long("log")
        .about("Prints log")
        .takes_value(true)
      )
      .arg(
        Arg::new("config")
        .short('c')
        .long("config")
        .about("Selects config file")
        .takes_value(true)
      )
    )

    .subcommand(App::new("GC")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Generate Configuration")
      .arg(
        Arg::new("verbose")
        .short('v')
        .long("verbose")
        .about("Prints verbosely")
      )
      .arg(
        Arg::new("logging")
        .short('l')
        .long("log")
        .about("Prints log")
        .takes_value(true)
      )
      .arg(
        Arg::new("config")
        .short('c')
        .long("config")
        .about("Selects config file")
        .takes_value(true)
      )
      .arg(
        Arg::new("force")
        .short('f')
        .long("force")
        .about("Overwrite configuration")
      )
    )

    .subcommand(App::new("AC")
      .version(crate_version!())
      .author(crate_authors!())
      .about("AutoCompletion")
      .arg(
        Arg::new("logging")
        .short('l')
        .long("log")
        .about("Prints log")
        .takes_value(true)
        .possible_values(&["debug", "info", "warn", "error"])
      )
      .arg(
        Arg::new("shell")
        .short('s')
        .long("shell")
        .about("Selects shell")
        .required(true)
        .takes_value(true)
        .possible_values(&["bash", "elvish", "fish", "powershell", "zsh"])
      )
      .arg(
        Arg::new("manual")
        .short('m')
        .long("manual")
        .about("Display instructions on how to install autocompletions")
      )
    )
      .arg(
        Arg::new("verbose")
        .short('v')
        .long("verbose")
        .about("Prints verbosely")
      )
      .arg(
        Arg::new("logging")
        .short('l')
        .long("log")
        .about("Prints log")
        .takes_value(true)
      )
      .arg(
        Arg::new("SHELL")
        .short('s')
        .long("shell")
        .about("Selects shell")
        .required(true)
        .takes_value(true)
        .possible_values(&["bash", "elvish", "fish", "powershell", "zsh"])
      )
    )

}

////////////////////////////////////////////////////////////////////////////////////////////////////
