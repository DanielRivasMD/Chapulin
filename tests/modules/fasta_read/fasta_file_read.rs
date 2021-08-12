////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::modules::fasta_read::fasta_file_read::fasta_reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! fasta_file_read {
  ( $func: ident; $assertion: ident; $key: expr; $val: expr ) => {
    #[test]
    fn $func() {
      let ref_seq = "tests/samples/dummy_ref.fa";
      let mutex_ref_seq = Arc::new(Mutex::new(HashMap::new()));
      let clone_mutex = Arc::clone(&mutex_ref_seq);

      // read fasta
      // observe that error is unwrap
      fasta_reader(ref_seq, mutex_ref_seq).unwrap();

      // assert
      $assertion!(clone_mutex.lock().unwrap().get($key), $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fasta_file_read!(test01; assert_eq; "1"; Some(&54.)); // value 1
fasta_file_read!(test02; assert_eq; "4"; Some(&14.)); // value 2
fasta_file_read!(test03; assert_eq; "10"; None); // no value
fasta_file_read!(fail01; assert_ne; "4"; None); // value 1
fasta_file_read!(fail02; assert_ne; "1"; Some(&504.)); // value 2
fasta_file_read!(fail03; assert_ne; "10"; Some(&0.)); // no value

////////////////////////////////////////////////////////////////////////////////////////////////////
