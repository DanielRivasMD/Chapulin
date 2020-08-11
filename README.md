
## _Chapulin_: Next-generation genomic mobile element and structural variant identification tool

![The Man in the Chapulin Hill](assests/chapulin.png)

## Table of contents

- [Overview](#overview)
- [Installation](#installation)
  - [Homebrew](#via-homebrew-for-macos)
  - [APT](#via-apt-for-debian-based-linux-distros)
  - [Cargo](#via-cargo-for-linux-windows-or-macos)
  - [GitHub](#from-github-release)
  - [Source](#building-from-source)
- [Documentation](#documentation)
  - [Usage](#usage)
  - [ME subcommand](#me-subcommand)
  - [SV subcommand](#sv-subcommand)
- [Examples](#examples)
  - [Config](#example-chapulin-config)
  - [ME subcommand](#example-chapulin-me-subcommand)
  - [SV subcommand](#example-chapulin-sv-subcommand)
- [For the curious](#chapulin-for-the-curious)
  - [Etimology](#etimology)
  - [Cultural reference](#cultural-reference)
- [Acknowledgements](#acknowledgements)
- [License](#license)


## Overview


Chapulin offers two different modes: 
  
  - Mobile Element identification (ME): 

  - Structural Variant identification (SV):


## Installation


<!-- TODO: -->
### Via Homebrew (for macOS)

Prerequisites:

- [Homebrew](https://brew.sh/)

```
brew install danielrivasmd/chapulin
```



<!-- TODO: -->
### Via APT (for Debian-based Linux distros)

```
curl -SsL https://fbecart.github.io/ppa/debian/KEY.gpg | sudo apt-key add -
sudo curl -SsL -o /etc/apt/sources.list.d/fbecart.list https://fbecart.github.io/ppa/debian/fbecart.list
sudo apt update
sudo apt install chapulin
```



<!-- TODO: -->
### Via Cargo (for Linux, Windows or macOS)

Prerequisites:

- [Rust toolchain](https://rustup.rs/)

```
cargo install chapulin
```



<!-- TODO: -->
### From Github release

Simply download the release binary for your operating system



### Building from source

Chapulin is written in Rust, so you'll need to grab a [Rust installation](https://rustup.rs/) in order to compile it.

To build Chapulin:

```
git clone https://github.com/DanielRivasMD/Chapulin
cd Chapulin
cargo build --release
./target/release/chapulin --version
Chapulin 0.1.0
```

To run the test suite, use:

```
cargo test
```


## Documentation

### Usage

Use `chapulin -h` or `chapulin --help` to display help on commandline. 

```
Chapulin 0.1.0
Daniel Rivas <danielrivasmd@gmail.com>

      
Chapulin: Mobile Element Identification
      
Software for mobile element identification in resequenced short-read data with a reference genome.
      

  Available subcommands are:
      
Mobile Element (ME): performs sequence similarity search to a customized mobile element library and insertion calls by
probability or a set threshold.
      
Structural Variant (SV): performs read selection based on alignment data and variant calls by probability or a set
threshold.
    

USAGE:
    chapulin [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    ME      Mobile Element Identification
    SV      Structural Variant Identification
    T       Testing
    help    Prints this message or the help of the given subcommand(s)
```

### ME subcommand



### SV subcommand



## Examples

<!-- TODO:
add additional example in example folder
 -->

### Example `chapulin` config

```toml
```
<!-- TODO:
 -->

### Example `chapulin ME` subcommand

```
chapulin ME -c <CONFIG>
```

### Example `chapulin SV` subcommand

```
chapulin SV -c <CONFIG>
```

## _Chapulin_ for the curious

### Etimology

The word _chapulin_ derives from Náhuatl _chapōlin_, where the compounds _chapā[nia]_ and _ōlli_ mean "to bounce" and "rubber", respectively. Thus meaning "insect that bounces like rubber".

### Cultural reference

Inhabitants and visitors of Mexico City will be familiar to the 'Chapulin' image for its reference to the beautiful "Chapultepec" or "Chapulin's hill" forest, castle and metro station. It also alludes to the delicious "chapulin's tacos" eaten in Central and South Mexico.


## Acknowledgements



## License

Chapulin is distributed under the terms of the GNU GENERAL PUBLIC LICENSE.

See [LICENSE](LICENSE) for details.
