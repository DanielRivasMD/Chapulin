
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

////////////////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: finish tests
// test private functions
#[cfg(test)]
mod priv_tests {
  use data_test::data_test;
  use crate::utils::structures::sv_chimeric_pair::SVChimericPair;
  use crate::utils::structures::sv_type::SVType;
  use super::{
    sv_deletion,
    sv_duplication,
    sv_inversion,
    sv_insertion,
    sv_translocation,
  };

  data_test! {

    fn test_deletion(pos1, pos2, exlen, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.pos = pos1;
      svchim.read2.chr_read.pos = pos2;

      assert!(super::sv_deletion(&mut svchim, exlen), expected);
    }
    - un_del (1000, 30000, 500, false)

    fn test_duplication(tl, fl1, fl2, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.tlen = tl;
      svchim.read1.chr_read.flag = fl1;
      svchim.read2.chr_read.flag = fl2;

      assert!(super::sv_duplication(&mut svchim), expected);
    }
    - un_dup (1, 123, 324, true)

    fn test_inversion(fl1, ch1, fl2, ch2, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.flag = fl1;
      svchim.read1.chr_read.chr = ch1.to_string();
      svchim.read2.chr_read.flag = fl2;
      svchim.read2.chr_read.chr = ch2.to_string();

      assert!(super::sv_inversion(&mut svchim), expected);
    }
    - un_inv (177, 1, 177, 1, false)

    fn test_insertion(fl1, fl2, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.flag = fl1;
      svchim.read2.chr_read.flag = fl2;

      assert!(super::sv_insertion(&mut svchim), expected);
    }
    - un_ins (123, 123, false)

    fn test_translocation(pos1, ch1, pos2, ch2, expected) => {

      // load values
      let mut svchim = super::SVChimericPair::new(super::SVType::None);
      svchim.read1.chr_read.pos = pos1;
      svchim.read1.chr_read.chr = ch1.to_string();
      svchim.read2.chr_read.pos = pos2;
      svchim.read2.chr_read.chr = ch2.to_string();

      assert!(super::sv_translocation(&mut svchim), expected);
    }
    - un_trans (10, 1, 809, 7, false)

  }

}

////////////////////////////////////////////////////////////////////////////////////////////////////
