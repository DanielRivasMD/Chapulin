////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::modules::chromosomal_loci::cl_aligned;

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! cl_aligned {
  ( $function: ident; $assertion: ident; $key: expr; $val: expr ) => {
    #[test]
    fn $function() {
      // declare files
      let cl_alignment = "tests/samples/cl_alignment.sam";
      let errata = "";

      // declare anchor registry
      let mutex_anchor_registry = Arc::new(Mutex::new(HashMap::new()));

      // declare chimeric chromosomal loci collection
      let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
      let clone_mutex = Arc::clone(&mutex_record_collection);

      // identify mobile elements
      // observe that error is unwrap
      cl_aligned::cl_mapper(
        cl_alignment,
        errata,
        mutex_record_collection,
        mutex_anchor_registry,
        0,
      )
      .unwrap();

      // assert
      $assertion!(clone_mutex.lock().unwrap().get($key), $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: update sample file

// no value
cl_aligned!(test00; assert_eq; "CS0001.1"; None);

// mount value through function

// mount value manually

////////////////////////////////////////////////////////////////////////////////////////////////////
