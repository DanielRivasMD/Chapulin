////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use std::collections::HashMap;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::modules::mobile_elements::me_aligned;
use genomic_structures::{
  ChrAnchorEnum,
  ExtraValuesEnum,
  MEAnchor,
  MEChimericPair,
  MEChimericRead,
  RawValues,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! me_aligned {
  ( $function: ident; $assertion: ident; $result: ident ; $key: expr; $val: expr; $mobel_id: expr; $mobel_size: expr ) => {
    #[test]
    fn $function() {
      // declare files
      let me_alignment = "tests/samples/me_alignment.sam";

      // declare mobile element library hashmap
      let mutex_me_collection = Arc::new(Mutex::new(HashMap::new()));

      // create mobile element library
      // let mobel_size = 1000.;
      mutex_me_collection
        .lock()
        .unwrap()
        .insert(String::from($mobel_id), $mobel_size);
      // .insert(String::from("cs100"), 1000.);

      // declare mutex
      let mutex_record_collection = Arc::new(Mutex::new(HashMap::new()));
      let clone_mutex = Arc::clone(&mutex_record_collection);

      // identify mobile elements
      // observe that error is unwrap
      me_aligned::me_identificator(
        me_alignment,
        mutex_me_collection,
        mutex_record_collection,
        0,
      )
      .unwrap();

      // assert
      $assertion!(
        clone_mutex.lock().unwrap().get($key),
        $result(&chimeric_pair_make($val))
      );
      // $assertion!($key, $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// read1: Vec<&str>,
// read2: Vec<&str>,
// SAM line reader
fn chimeric_pair_make(flines: Vec<Vec<&str>>) -> MEChimericPair {
  // declare mobile element chimeric pair
  let mut chimeric_pair = MEChimericPair::new();

  // load raw values
  chimeric_pair.read1 = raw_values_load(&flines[0]);
  chimeric_pair.read2 = raw_values_load(&flines[1]);

  // load chimeric read

  // make chr anchor
  // chimeric_pair.chranch = ChrAnchorEnum::Read2;

  // // load read1 raw values
  // let raw_values = RawValues::load(read1).unwrap();

  // // load read2 raw values
  // let raw_values = RawValues::load(read2).unwrap();

  // raw_values.update(flines).unwrap();
  // let mut cigar = CIGAR::new();

  // let mut chimeric_read1 = MEChimericRead::new();
  // let mut chimeric_read2 = MEChimericRead::new();
  // let mut chranch = ChrAnchorEnum::new();

  chimeric_pair
}

// raw_values: RawValues,
// chimeric_pair: MEChimericPair,
fn raw_values_load(flines: &[&str]) -> MEChimericRead {
  // load raw values
  let mut raw_values = RawValues::load(flines.to_vec()).unwrap();

  raw_values.extra = ExtraValuesEnum::MobelSize(1000.);

  // construct chimeric read
  let mut chimeric_read = MEChimericRead::new();

  // mobile element
  chimeric_read.me_read.push(MEAnchor::load(
    raw_values.cigar.clone(),
    raw_values.flag,
    raw_values.scaffold.clone(),
    raw_values.orientation.clone(),
    raw_values.position,
    raw_values.extra_get(),
  ));

  chimeric_read.quality = raw_values.quality;
  chimeric_read.sequence = raw_values.sequence;

  chimeric_read
}

// me_aligned!(test01; assert_eq; CIGAR::new(); CIGAR::new());
// me_aligned!(fail01; assert_ne; CIGAR::load("100M", 100).unwrap();
// CIGAR::new());

// me_aligned!(test01; assert_eq; "SRR556146.1"; None; ""; ""; 0.);
me_aligned!(test02; assert_eq; Some; "CS0001.1";
  vec![
    vec!["CS0001.1", "75", "REF_cs100", "1", "37", "100M", "=", "150", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    vec!["CS0001.1", "135", "REF_cs100", "150", "37", "100M", "=", "1", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"]
    ];
"REF_cs100";
1000.
);

////////////////////////////////////////////////////////////////////////////////////////////////////
