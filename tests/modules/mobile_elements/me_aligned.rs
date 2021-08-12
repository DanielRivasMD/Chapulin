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
  BreakPoint,
  ChrAnchorEnum,
  ExtraValuesEnum,
  MEAnchor,
  MEChimericPair,
  MEChimericRead,
  OrientationEnum,
  RawValues,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! me_aligned {
  ( $function: ident; $assertion: ident; $key: expr; $val: expr; $mobel_id: expr; $mobel_size: expr ) => {
    #[test]
    fn $function() {
      // declare files
      let me_alignment = "tests/samples/me_alignment.sam";

      // declare mobile element library hashmap
      let mutex_me_collection = Arc::new(Mutex::new(HashMap::new()));

      // create mobile element library
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
      $assertion!(clone_mutex.lock().unwrap().get($key), $val);
      // $assertion!($key, $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: integrate chimeric pair build into macro
me_aligned!(test01; assert_ne; "CS0001.1"; None; ""; 0.);
me_aligned!(test02; assert_eq; "CS0001.1";
  Some(&chimeric_pair_build(&[
    &["CS0001.1", "75", "REF_cs100", "1", "37", "100M", "=", "150", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    &["CS0001.1", "135", "REF_cs100", "150", "37", "100M", "=", "1", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"]
    ]));
"REF_cs100";
1000.
);
me_aligned!(test03; assert_eq; "CS0001.1";
  Some(&MEChimericPair{
    read1: MEChimericRead{
      breakpoint: BreakPoint{
        sequence: String::new(),
        coordinate: 0,
      },
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          cigar: CIGAR{
            align: vec![100],
            del: vec![],
            ins: vec![],
            lclip: 0,
            left_boundry: 1,
            rclip: 0,
            right_boundry: 101,
            signature: String::from("100M"),
          },
          flag: 75,
          mobel: String::from("REF_cs100"),
          orientation: OrientationEnum::None,
          position: 1,
          size: 1000.0
        }
      ],
      quality: 37,
      sequence: String::from("AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN"),
    },
    read2: MEChimericRead{
      breakpoint: BreakPoint{
        sequence: String::new(),
        coordinate: 0
      },
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          cigar: CIGAR{
            align: vec![100],
            del: vec![],
            ins: vec![],
            lclip: 0,
            left_boundry: 150,
            rclip: 0,
            right_boundry: 250,
            signature: String::from("100M"),
          },
          flag: 135,
          mobel: String::from("REF_cs100"),
          orientation: OrientationEnum::None,
          position: 150,
          size: 1000.0
        }
      ],
      quality: 37,
      sequence: String::from("TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC"),
    },
    chranch: ChrAnchorEnum::None,
  });
"REF_cs100";
1000.
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// SAM line reader
fn chimeric_pair_build(flines: &[&[&str]]) -> MEChimericPair {
  // declare mobile element chimeric pair
  let mut chimeric_pair = MEChimericPair::new();

  // load chimeric read
  chimeric_pair.read1 = chimeric_read_build(&flines[0]);
  chimeric_pair.read2 = chimeric_read_build(&flines[1]);

  chimeric_pair
}

fn chimeric_read_build(flines: &[&str]) -> MEChimericRead {
  // load raw values
  let mut raw_values = RawValues::load(flines.to_vec()).unwrap();

  // hardcoded
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

////////////////////////////////////////////////////////////////////////////////////////////////////
