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

fn mux(ref_seq: &str) -> Arc<Mutex<HashMap<String, f64>>> {
  // declare arc mutex & clone
  let mutex_ref_seq = Arc::new(Mutex::new(HashMap::new()));
  let clone_mutex = Arc::clone(&mutex_ref_seq);

  // read fasta
  // observe that error is unwrap
  fasta_reader(ref_seq, mutex_ref_seq).unwrap();

  clone_mutex
}

////////////////////////////////////////////////////////////////////////////////////////////////////

data_test! {

  fn test_fasta_reader(ref_seq, key, val) => {
    // declare values
    let clone_mutex = super::mux(ref_seq);
    // assert
    assert_eq!(clone_mutex.lock().unwrap().get(key), Some(&val));
  }

  - _01("tests/samples/dummy_ref.fa", "1", 54.)
  - _02("tests/samples/dummy_ref.fa", "4", 14.)

  fn fail_fasta_reader(ref_seq, key, val) => {
    // declare values
    let clone_mutex = super::mux(ref_seq);
    // assert
    assert_ne!(clone_mutex.lock().unwrap().get(key), Some(&val));
  }

  - _01("tests/samples/dummy_ref.fa", "1", 50.)
  - _02("tests/samples/dummy_ref.fa", "4", 4.)

}

////////////////////////////////////////////////////////////////////////////////////////////////////
