
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use data_test::data_test;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::settings::constants::NO_FDR;
use chapulin::utils::functions::thresholder::thresholder;

////////////////////////////////////////////////////////////////////////////////////////////////////


data_test! {

  fn test_thresholder(
    pop_reads,
    chromosome_size,
    false_discovery_tolerance,
    read_hm_k,
    read_hm_v,
    psize,
    expected
  ) => {

    let mut hm = std::collections::HashMap::new();
    hm.insert(
      read_hm_k,
      read_hm_v,
    );

    assert_eq!(
      super::thresholder(
        pop_reads,
        chromosome_size,
        false_discovery_tolerance,
        &hm,
        psize,
      ),
      expected
    );
  }
  - talfa (0., 1000., 0.001, "1".to_string(), vec!["uno".to_string(), "dos".to_string(), "tres".to_string(),], super::NO_FDR, 3)
  // TODO: add more tests

}

////////////////////////////////////////////////////////////////////////////////////////////////////
