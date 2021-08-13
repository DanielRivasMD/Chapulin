////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use std::sync::Arc;

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use chapulin::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::modules::fasta_read::fasta_file_read;

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! fasta_read {
  ( $function: ident; $assertion: ident; $key: expr; $val: expr ) => {
    #[test]
    fn $function() {
      // declare file
      let ref_seq = "tests/samples/dummy_ref.fa";

      // declare mutex
      let mutex_ref_seq = alias::arc_map();
      let clone_mutex = alias::arc_clone(&mutex_ref_seq);

      // read fasta
      // observe that error is unwrap
      fasta_file_read::fasta_read(ref_seq, mutex_ref_seq).unwrap();

      // assert
      $assertion!(clone_mutex.lock().unwrap().get($key), $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// tests

// no value
fasta_read!(test00; assert_eq; "10"; None);

// value 1
fasta_read!(test01; assert_eq; "1"; Some(&54.));

// value 2
fasta_read!(test02; assert_eq; "4"; Some(&14.));

// no value
fasta_read!(fail03; assert_ne; "10"; Some(&0.));

// value 1
fasta_read!(fail01; assert_ne; "4"; None);

// value 2
fasta_read!(fail02; assert_ne; "1"; Some(&504.));

////////////////////////////////////////////////////////////////////////////////////////////////////
