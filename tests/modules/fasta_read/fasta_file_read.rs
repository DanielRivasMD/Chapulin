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

macro_rules! fastread {
  ( $func: ident; $assertion: ident; $key: expr; $val: expr ) => {
    #[test]
    fn $func() {
      // let ref_seq = "../../samples/dummy_ref.fa";
      let ref_seq = "tests/samples/dummy_ref.fa";
      let mutex_ref_seq = Arc::new(Mutex::new(HashMap::new()));
      let clone_mutex = Arc::clone(&mutex_ref_seq);

      // read fasta
      // observe that error is unwrap
      fasta_reader(ref_seq, mutex_ref_seq).unwrap();

      // assert
      $assertion!(clone_mutex.lock().unwrap().get($key), Some(&$val));
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fastread!(test01; assert_eq; "1"; 54.);
fastread!(test02; assert_eq; "4"; 14.);
fastread!(fail01; assert_ne; "4"; 104.);
fastread!(fail02; assert_ne; "1"; 504.);
fastread!(fail03; assert_ne; "10"; 0.);

////////////////////////////////////////////////////////////////////////////////////////////////////
