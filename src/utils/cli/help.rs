////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::{crate_authors, crate_version, App, AppSettings, Arg};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn cli_chapulin() -> App<'static> {

  App::new("chapulin")
    .version(crate_version!())
    .author(crate_authors!())
    .setting(AppSettings::ArgRequiredElseHelp)
    .setting(AppSettings::ColoredHelp)
    .about("
      \nChapulin: Mobile Element Identification
      \nSoftware for mobile element identification in resequenced short-read data with a reference genome.
      \n\n\tAvailable subcommands are:
      \nMobile Element (ME): performs sequence similarity search to a customized mobile element library and insertion calls by probability or a set threshold. Aliases: 'me', 'MobileElement'.
      \nStructural Variant (SV): performs read selection based on alignment data and variant calls by probability or a set threshold. Aliases: 'sv', 'StructuralVariant'.
      \nCache Registering (CR): checks for reference genome and mobile element library cache in configuration directory. In case caches are not found, reads files and writes cache. Aliases: 'cr', 'CacheRegistering'.
      \nGenerate Configuration (GC): generates a configuration template. Observe that not all values from config file are used at all times. Aliases: 'gc', 'GenerateConfiguration'.
      \nAutoCompletion (AC): generates autocompletions to stdout for your shell. Pipe into a file and install to get help when using Chapulin. See `chapulin AC --manual` for details. Aliases: 'ac', AutoCompletion'.
    ")

    .subcommand(App::new("ME")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Mobile Element Identification")
      .aliases(&["me", "MobileElement"])
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
        .possible_values(&["debug", "info", "warn", "error"])
      )
      .arg(
        Arg::new("config")
        .short('c')
        .long("config")
        .about("Selects config file")
        .required(true)
        .takes_value(true)
      )
      .arg(
        Arg::new("dry")
        .long("dry-run")
        .about("Display settings without running command")
      )
      .arg(
        Arg::new("chralign")
        .short('a')
        .long("alignment")
        .about("Selects chromosomal alignment (default: paired)")
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
      .aliases(&["sv", "StructuralVariant"])
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
        .possible_values(&["debug", "info", "warn", "error"])
      )
      .arg(
        Arg::new("config")
        .short('c')
        .long("config")
        .about("Selects config file")
        .required(true)
        .takes_value(true)
      )
      .arg(
        Arg::new("dry")
        .long("dry-run")
        .about("Display settings without running command")
      )
    )

    .subcommand(App::new("CR")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Cache Registering")
      .aliases(&["cr", "CacheRegistering"])
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
        .possible_values(&["debug", "info", "warn", "error"])
      )
      .arg(
        Arg::new("config")
        .short('c')
        .long("config")
        .about("Selects config file")
        .required(true)
        .takes_value(true)
      )
      .arg(
        Arg::new("dry")
        .long("dry-run")
        .about("Display settings without running command")
      )
    )

    .subcommand(App::new("GC")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Generate Configuration")
      .aliases(&["gc", "GenerateConfiguration"])
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
        .possible_values(&["debug", "info", "warn", "error"])
      )
      .arg(
        Arg::new("config")
        .short('c')
        .long("config")
        .about("Selects config file")
        .required(true)
        .takes_value(true)
      )
      .arg(
        Arg::new("force")
        .short('f')
        .long("force")
        .about("Use excesive force! Overwrite configuration")
      )
      .arg(
        Arg::new("dry")
        .long("dry-run")
        .about("Display settings without running command")
      )
    )

    .subcommand(App::new("AC")
      .version(crate_version!())
      .author(crate_authors!())
      .about("AutoCompletion")
      .aliases(&["ac", "AutoCompletion"])
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
