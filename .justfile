####################################################################################################

_default:
  @just --list

####################################################################################################

# print justfile
@show:
  bat .justfile --language make

####################################################################################################

# edit justfile
@edit:
  micro .justfile

####################################################################################################

# aliases

####################################################################################################

# check
@check:
  cargo watch --clear --why --exec check

####################################################################################################

# test list
@test-list:
  cargo watch --clear --shell 'cargo nextest list'

####################################################################################################

# test run
@test-run:
  cargo watch --clear --shell 'cargo nextest run'

####################################################################################################

# build
@debug:
  cargo build

####################################################################################################

# build release
@build:
  cargo build --release

####################################################################################################

# format rustfmt
@fmt:
  rustfmt +nightly src/*rs

####################################################################################################

# test debug help
h:
  cargo build && ./target/debug/chapulin --help

################################################################################

# test configuration generator
gch:
  cargo build && ./target/debug/chapulin GC --help

################################################################################

# test mobile element module
meh:
  cargo build && ./target/debug/chapulin ME --help

################################################################################

# run mobile element module as single-end
mes:
  cargo build && ./target/debug/chapulin ME -v --log info -c ~/chapulinTest/test.toml -a single -d 100

################################################################################

# run mobile element module as paired-end
mep:
  cargo build && ./target/debug/chapulin ME -v --log info -c ~/chapulinTest/test.toml -a paired

################################################################################

# run structural variant module
sv:
  cargo build && ./target/debug/chapulin SV -v --log info -c ~/chapulinTest/test.toml

################################################################################

# run cache registering module
cr:
  cargo build && ./target/debug/chapulin CR -v --log info -c ~/chapulinTest/test.toml

################################################################################

# run testing
t:
  cargo build && ./target/debug/chapulin T -v --log info -c ~/chapulinTest/test.toml

################################################################################

# # run generate completions
# rungc:
#   cargo build && ./target/debug/chapulin GC -v --log info -f

# ################################################################################

# # run test module
# runt:
#   cargo build && ./target/debug/chapulin T -v -c ~/chapulinTest/test.toml

# ################################################################################

# # build release
# build:
#   cargo build --release

# ################################################################################

# # run cargo test & write to out/testout
# test:
#   script -q /dev/null cargo test | tee out/testout

# ################################################################################

# # run cargo clippy
# debug:
#   cargo clippy

# ################################################################################

# # try dry-run
# dry:
#   cargo build && ./target/debug/chapulin ME --dry-run -c ~/chapulinTest/test.toml

# ################################################################################

# # deliver repository to Uppmax
# @ hermesUppmax:
#   rsync -azvhP --delete ~/Factorem/Chapulin drivas@rackham.uppmax.uu.se:/home/drivas/Factorem

# ################################################################################

# # link Chapulin to PATH
# @ link:
#   ln -svf $HOME/Factorem/Chapulin/target/debug/chapulin $HOME/bin/toolLinks/

# ################################################################################

# # clean rustfmt
# @ clean:
#   fd tmp --exec rm -v {}

################################################################################
