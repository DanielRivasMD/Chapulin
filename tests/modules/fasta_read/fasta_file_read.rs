////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use data_test::data_test;
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::modules::fasta_read::fasta_file_read::fasta_reader;

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_fasta_reader(ref_seq, key, val) => {
    // declare arc mutex & clone
    let mutex_ref_seq = super::Arc::new(super::Mutex::new(super::HashMap::new()));
    let clone_mutex = super::Arc::clone(&mutex_ref_seq);

    // read fasta
    // observe that error is unwrap
    super::fasta_reader(ref_seq, mutex_ref_seq).unwrap();

    // assert
    assert_eq!(clone_mutex.lock().unwrap().get(key), Some(&val));
  }

  - fasta1("tests/samples/dummy_ref.fa", "1", 54.)
  - fasta2("tests/samples/dummy_ref.fa", "4", 14.)

  fn fail_fasta_reader(ref_seq, key, val) => {
    // declare arc mutex & clone
    let mutex_ref_seq = super::Arc::new(super::Mutex::new(super::HashMap::new()));
    let clone_mutex = super::Arc::clone(&mutex_ref_seq);

    // read fasta
    // observe that error is unwrap
    super::fasta_reader(ref_seq, mutex_ref_seq).unwrap();

    // assert
    assert_ne!(clone_mutex.lock().unwrap().get(key), Some(&val));
  }

  - fasta1("tests/samples/dummy_ref.fa", "1", 50.)
  - fasta2("tests/samples/dummy_ref.fa", "4", 4.)

}

////////////////////////////////////////////////////////////////////////////////////////////////////
