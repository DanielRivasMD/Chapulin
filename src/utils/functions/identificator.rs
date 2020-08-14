
////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::{
  settings::{
    constants::TRANSLOCATION_DISTANCE,
  },
  utils::{
    functions::{
      flag_interpretor::interpretor,
    },
    structures::{
      sv_chimeric_pair::SVChimericPair,
      sv_type::SVType, 
    },
  },
};

////////////////////////////////////////////////////////////////////////////////////////////////////


fn sv_deletion(pair: &mut SVChimericPair, expected_tlen: i32) -> bool {
  let tlen = pair.read1.chr_read.pos - pair.read2.chr_read.pos;
  if tlen.abs() > expected_tlen {
    pair.svtag = SVType::Deletion;
    true
  } else {
    false
  }
}

fn sv_duplication(pair: &mut SVChimericPair) -> bool {
  if pair.read1.chr_read.tlen > 0 && ! interpretor(pair.read1.chr_read.flag, 5) && ! interpretor(pair.read2.chr_read.flag, 5) {
    pair.svtag = SVType::Duplication;
    true
  } else {
    false
  }
}

fn sv_inversion(pair: &mut SVChimericPair) -> bool {
  if interpretor(pair.read1.chr_read.flag, 5) == interpretor(pair.read2.chr_read.flag, 5) && (pair.read1.chr_read.chr == pair.read2.chr_read.chr) {
    pair.svtag = SVType::Inversion;
    true
  } else {
    false
  }
}

fn sv_insertion(pair: &mut SVChimericPair) -> bool {
  if interpretor(pair.read1.chr_read.flag, 3) || interpretor(pair.read2.chr_read.flag, 3) {
    pair.svtag = SVType::Insertion;
    true
  } else {
    false
  }
}

fn sv_translocation(pair: &mut SVChimericPair) -> bool {
  let tlen = pair.read1.chr_read.pos - pair.read2.chr_read.pos;
  if tlen.abs() > TRANSLOCATION_DISTANCE || pair.read1.chr_read.chr != pair.read2.chr_read.chr {
    pair.svtag = SVType::Translocation;
    true
  } else {
    false
  }
}

pub fn identificator(pair: &mut SVChimericPair, expected_tlen: i32) -> bool {

  let mut psw = vec![];

  // evaluate read pairs
  psw.push(sv_deletion(pair, expected_tlen));
  psw.push(sv_duplication(pair));
  psw.push(sv_inversion(pair));
  psw.push(sv_insertion(pair));
  psw.push(sv_translocation(pair));
  // TODO: correct BUG. variant are called simultaneous
  ic!(psw);

  psw.contains(&true)
}

////////////////////////////////////////////////////////////////////////////////////////////////////

pub trait SVIdentificator {
  fn identificator(self, expected_tlen: i32) -> bool;
}

