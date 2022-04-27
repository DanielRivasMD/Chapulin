////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  ChrAnchor,
  MEChimericPair,
  MEChimericRead,
  OrientationEnum,
  StrandDirection,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use chapulin::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::modules::peak_identification::pi_me_mapping::pi_me_identifier;

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! test_pi_me_mapping {
  ( $function: ident;
    vargs ... $($no_read: expr => $mapq: expr),+;
    expect |> $expect: expr;
  ) => {
    #[test]
    fn $function() {
      // declare
      let key = "chrT".to_string();

      // declare mutex
      // let amx_chr_registry = alias::arc_map();
      let amx_chr_library = alias::arc_map();
      let amx_dir_registry = alias::arc_map();
      let amx_me_record = alias::arc_map();

      // // create scaffold
      amx_dir_registry.lock().unwrap().insert(key.clone(), StrandDirection::new());

      // insert read ids variadic
      $(
          // // fill scaffold with read ids
          // if let Some(an_registry) = amx_chr_registry.lock().unwrap().get_mut(&key) {
          //   an_registry.push($no_read.to_string());
          // }

          // fill scaffold with read ids
          if let Some(direction) = amx_dir_registry.lock().unwrap().get_mut(&key) {
            direction.push($dir_strand);
          }

          // fill chimeric pairs
          amx_me_record.lock().unwrap().insert($no_read.to_string(), MEChimericPair::new());

          // assing mapping quality
          if let Some(current_record) = amx_me_record.lock().unwrap().get_mut($no_read) {
            current_record.read1.chr_read = vec![ChrAnchor::new()];
            current_record.read1.chr_read[0].mapq = $mapq;
            // current_record.read1.chr_read[0].
          }
      )+

      // declare assertion clone
      let camx_me_record_as = alias::arc_clone(&amx_me_record);

      let out_dir = "out/";
      let output = format!("{}{}.csv", out_dir, key);

      // filter mapq
      // threshold
      pi_me_identifier(
        &output,
        // amx_chr_registry,
        amx_chr_library,
        amx_dir_registry,
        amx_me_record,
      )
      .expect("Error occured at peak mobile element identificator!");

      // assert
      assert!(camx_me_record_as.lock().unwrap().contains_key(&$expect));
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// test
test_pi_me_mapping!(test00;
  vargs ... "READ_1" => 60;
  expect |> "READ_1".to_string();
);

////////////////////////////////////////////////////////////////////////////////////////////////////
