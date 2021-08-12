////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use clap::{
  crate_authors,
  crate_version,
  App,
  AppSettings,
  Arg,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: set up cmd aliases
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
      .visible_aliases(&["me", "MobileElement"])
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
      .arg(
        Arg::new("debug")
        .short('d')
        .long("debug")
        .about("Print values for debugging")
        .required(true)
        .takes_value(true)
        .default_value("0")
      )
    )

    .subcommand(App::new("SV")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Structural Variant Identification")
      .visible_aliases(&["sv", "StructuralVariant"])
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
      .visible_aliases(&["cr", "CacheRegistering"])
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
      .visible_aliases(&["gc", "GenerateConfiguration"])
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
      .visible_aliases(&["ac", "AutoCompletion"])
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

    .subcommand(App::new("T")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Testing")
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
    )
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: example on how to take argument from default, cli | file
// fn main() {
//     let matches = App::new("MyApp")
//         .version("0.1.0")
//         .about("Example for StackOverflow")
//         .arg(
//             Arg::with_name("config")
//                 .short("c")
//                 .long("config")
//                 .value_name("FILE")
//                 .help("Sets a custom config file"),
//         )
//         .arg(
//             Arg::with_name("example")
//                 .short("e")
//                 .long("example")
//                 .help("Sets an example parameter")
//                 .default_value("default_value")
//                 .takes_value(true),
//         )
//         .get_matches();
//
//     let mut value = String::new();
//
//     if let Some(c) = matches.value_of("config") {
//         let file = File::open(c);
//         match file {
//             Ok(mut f) => {
//                 // Note: I have a file `config.txt` that has contents
// `file_value`                 f.read_to_string(&mut value).expect("Error
// reading value");             }
//             Err(_) => println!("Error reading file"),
//         }
//
//         // Note: this lets us override the config file value with the
//         // cli argument, if provided
//         if matches.occurrences_of("example") > 0 {
//             value = matches.value_of("example").unwrap().to_string();
//         }
//     } else {
//         value = matches.value_of("example").unwrap().to_string();
//     }
//
//     println!("Value for config: {}", value);
// }
