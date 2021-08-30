////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
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

// aliases
use chapulin::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::modules::mobile_elements::me_aligned;

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: write a file to load mobile elements from
// test mobile element modules overall performance
// by testing controller function `me_identificator`
macro_rules! test_me_aligned {
  ( $function: ident;
    mobile |> $mobel_id: expr, $mobel_size: expr;
    params |> $key: expr,  $val: expr;
  ) => {
    #[test]
    fn $function() {
      // declare files
      let me_alignment = "tests/samples/me_alignment.sam";

      // declare mobile element library
      let amx_me_library = alias::arc_map();

      // insert mobile element library
      amx_me_library
        .lock()
        .unwrap()
        .insert($mobel_id, $mobel_size);

      // declare chimeric mobile element collection
      let amx_me_record = alias::arc_map();

      // declare chimeric mobile element clone
      let camx_me_record_me = alias::arc_clone(&amx_me_record);

      // identify mobile elements
      me_aligned::me_identificator(
        me_alignment,
        amx_me_library,
        amx_me_record,
        0,
      )
      .expect("Error occured at mobile element identificator!");

      // assert
      assert_eq!(camx_me_record_me.lock().unwrap().get($key), $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// SAM line reader
fn chimeric_pair_build(
  flines: &[&[&str]],
  mobel_size: f64,
) -> MEChimericPair {
  // declare mobile element chimeric pair
  let mut chimeric_pair = MEChimericPair::new();

  // load chimeric read
  chimeric_pair.read1 = chimeric_read_build(&flines[0], mobel_size);
  chimeric_pair.read2 = chimeric_read_build(&flines[1], mobel_size);

  // tag mobile element anchors iteratively
  chimeric_pair.tag();

  // return
  chimeric_pair
}

fn chimeric_read_build(
  flines: &[&str],
  mobel_size: f64,
) -> MEChimericRead {
  // load raw values
  let mut raw_values = RawValues::load(flines.to_vec()).unwrap();

  // load mobile element size
  raw_values.extra = ExtraValuesEnum::MobelSize(mobel_size);

  // construct chimeric read
  let mut chimeric_read = MEChimericRead::new();

  // load mapq & sequence
  chimeric_read.sequence = raw_values.sequence.clone();

  // load mobile element
  chimeric_read.me_read.push(MEAnchor::load(
    raw_values.cigar.clone(),
    raw_values.flag,
    raw_values.scaffold.clone(),
    raw_values.orientation.clone(),
    raw_values.position,
    raw_values.get_extra(),
  ));

  // calculate break point iteratively
  chimeric_read.me_read.iter_mut().for_each(|me_anchor| {
    me_anchor.calculate_break_point(&raw_values.sequence)
  });

  // return
  chimeric_read
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// no value
//////////////////////////////////////////////////

// no read id value
test_me_aligned!(test00;
  mobile |> "RANDOM_ME".to_string(), 1000.;
  params |> "RANDOM_ID", None;
);

// not available mobile element
test_me_aligned!(test01;
  mobile |> "MOBEL_NA".to_string(), 1000000.;
  params |> "MOBEL_NA", None;
);

// drop upstream
test_me_aligned!(test02;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_DROP", None;
);

// drop downstream
test_me_aligned!(test03;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_DROP", None;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream keep
//////////////////////////////////////////////////

// mount value through function
test_me_aligned!(test11;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP1",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP1", "167", "mobel11000", "101", "0", "*", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["UPSTREAM_KEEP1", "91", "mobel11000", "101", "60", "100M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test12;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 101,
            left_clip: 0,
            right_boundry: 200,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 91,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 101,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 167,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 101,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value through function
test_me_aligned!(test13;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP2",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP2", "91", "mobel11000", "101", "60", "100M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["UPSTREAM_KEEP2", "167", "mobel11000", "101", "0", "*", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test14;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 101,
            left_clip: 0,
            right_boundry: 200,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 91,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 101,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 167,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 101,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream keep
//////////////////////////////////////////////////

// mount value through function
test_me_aligned!(test16;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP1",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP1", "135", "mobel11000", "10751", "0", "*", "", "", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["DOWNSTREAM_KEEP1", "75", "mobel11000", "10751", "60", "100M", "", "", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test17;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10751,
            left_clip: 0,
            right_boundry: 10850,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 75,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10751,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 135,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 10751,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value through function
test_me_aligned!(test18;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP2",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP2", "75", "mobel11000", "10751", "60", "100M", "", "", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["DOWNSTREAM_KEEP2", "135", "mobel11000", "10751", "0", "*", "", "", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test19;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10751,
            left_clip: 0,
            right_boundry: 10850,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 75,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10751,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 135,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 10751,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream keep break anchor
//////////////////////////////////////////////////

// mount value through function
test_me_aligned!(test21;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_ANCHOR1",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_BREAK_ANCHOR1", "167", "mobel11000", "1", "0", "*", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["UPSTREAM_KEEP_BREAK_ANCHOR1", "91", "mobel11000", "1", "60", "50S50M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test22;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_ANCHOR1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGAC".to_string(),
            coordinate: 50.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: -49,
            left_clip: 50,
            right_boundry: 50,
            rigth_clip: 0,
            signature: "50S50M".to_string(),
          },
          flag: 91,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 167,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value through function
test_me_aligned!(test23;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_ANCHOR2",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_BREAK_ANCHOR2", "91", "mobel11000", "1", "60", "50S50M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["UPSTREAM_KEEP_BREAK_ANCHOR2", "167", "mobel11000", "1", "0", "*", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test24;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_ANCHOR2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGAC".to_string(),
            coordinate: 50.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: -49,
            left_clip: 50,
            right_boundry: 50,
            rigth_clip: 0,
            signature: "50S50M".to_string(),
          },
          flag: 91,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 167,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream keep break anchor
//////////////////////////////////////////////////

// mount value through function
test_me_aligned!(test26;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_ANCHOR1",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_BREAK_ANCHOR1", "135", "mobel11000", "10951", "0", "*", "=", "0", "-100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["DOWNSTREAM_KEEP_BREAK_ANCHOR1", "75", "mobel11000", "10951", "60", "50M50S", "=", "0", "100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test27;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_ANCHOR1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "TTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
            coordinate: -49.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10951,
            left_clip: 0,
            right_boundry: 11050,
            rigth_clip: 50,
            signature: "50M50S".to_string(),
          },
          flag: 75,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 135,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value through function
test_me_aligned!(test28;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_ANCHOR2",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_BREAK_ANCHOR2", "75", "mobel11000", "10951", "60", "50M50S", "=", "0", "100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["DOWNSTREAM_KEEP_BREAK_ANCHOR2", "135", "mobel11000", "10951", "0", "*", "=", "0", "-100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test29;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_ANCHOR2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "TTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
            coordinate: -49.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10951,
            left_clip: 0,
            right_boundry: 11050,
            rigth_clip: 50,
            signature: "50M50S".to_string(),
          },
          flag: 75,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 135,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream keep break mate
//////////////////////////////////////////////////

// mount value through function
test_me_aligned!(test31;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_MATE1",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_BREAK_MATE1", "163", "mobel11000", "1", "60", "50S50M", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["UPSTREAM_KEEP_BREAK_MATE1", "83", "mobel11000", "151", "60", "100M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test32;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_MATE1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 151,
            left_clip: 0,
            right_boundry: 250,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 83,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 151,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCC".to_string(),
            coordinate: 50.
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: -49,
            left_clip: 50,
            right_boundry: 50,
            rigth_clip: 0,
            signature: "50S50M".to_string(),
          },
          flag: 163,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value through function
test_me_aligned!(test33;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_MATE2",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_BREAK_MATE2", "83", "mobel11000", "151", "60", "100M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["UPSTREAM_KEEP_BREAK_MATE2", "163", "mobel11000", "1", "60", "50S50M", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test34;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_MATE2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 151,
            left_clip: 0,
            right_boundry: 250,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 83,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 151,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCC".to_string(),
            coordinate: 50.
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: -49,
            left_clip: 50,
            right_boundry: 50,
            rigth_clip: 0,
            signature: "50S50M".to_string(),
          },
          flag: 163,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream keep break mate
//////////////////////////////////////////////////

// mount value through function
test_me_aligned!(test36;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_MATE1",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_BREAK_MATE1", "147", "mobel11000", "10951", "60", "50M50S", "", "", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["DOWNSTREAM_KEEP_BREAK_MATE1", "99", "mobel11000", "10851", "60", "100M", "", "", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test37;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_MATE1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10851,
            left_clip: 0,
            right_boundry: 10950,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 99,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10851,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "AGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
            coordinate: -49.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10951,
            left_clip: 0,
            right_boundry: 11050,
            rigth_clip: 50,
            signature: "50M50S".to_string(),
          },
          flag: 147,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value through function
test_me_aligned!(test38;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_MATE2",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_BREAK_MATE2", "99", "mobel11000", "10851", "60", "100M", "", "", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["DOWNSTREAM_KEEP_BREAK_MATE2", "147", "mobel11000", "10951", "60", "50M50S", "", "", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test39;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_MATE2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10851,
            left_clip: 0,
            right_boundry: 10950,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 99,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10851,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "AGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
            coordinate: -49.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10951,
            left_clip: 0,
            right_boundry: 11050,
            rigth_clip: 50,
            signature: "50M50S".to_string(),
          },
          flag: 147,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream keep break mate
//////////////////////////////////////////////////

// mount value through function
test_me_aligned!(test41;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_DOUBLE1",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_BREAK_DOUBLE1", "163", "mobel11000", "1", "60", "50S50M", "=", "0", "100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["UPSTREAM_KEEP_BREAK_DOUBLE1", "83", "mobel11000", "1", "60", "30S70M", "=", "0", "-100", "GCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTNTTTGTATTTTTATTAGAGAC", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test42;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_DOUBLE1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "GCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGAC".to_string(),
            coordinate: 30.,
          },
          cigar: CIGAR{
            align: vec![70],
            deletion: vec![],
            insertion: vec![],
            left_boundry: -29,
            left_clip: 30,
            right_boundry: 70,
            rigth_clip: 0,
            signature: "30S70M".to_string(),
          },
          flag: 83,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "GCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTNTTTGTATTTTTATTAGAGAC".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGAC".to_string(),
            coordinate: 50.
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: -49,
            left_clip: 50,
            right_boundry: 50,
            rigth_clip: 0,
            signature: "50S50M".to_string(),
          },
          flag: 163,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value through function
test_me_aligned!(test43;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_DOUBLE2",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_BREAK_DOUBLE2", "83", "mobel11000", "1", "60", "30S70M", "=", "0", "-100", "GCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTNTTTGTATTTTTATTAGAGAC", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["UPSTREAM_KEEP_BREAK_DOUBLE2", "163", "mobel11000", "1", "60", "50S50M", "=", "0", "100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test44;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_DOUBLE2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "GCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGAC".to_string(),
            coordinate: 30.,
          },
          cigar: CIGAR{
            align: vec![70],
            deletion: vec![],
            insertion: vec![],
            left_boundry: -29,
            left_clip: 30,
            right_boundry: 70,
            rigth_clip: 0,
            signature: "30S70M".to_string(),
          },
          flag: 83,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "GCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTNTTTGTATTTTTATTAGAGAC".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGAC".to_string(),
            coordinate: 50.
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: -49,
            left_clip: 50,
            right_boundry: 50,
            rigth_clip: 0,
            signature: "50S50M".to_string(),
          },
          flag: 163,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream keep break mate
//////////////////////////////////////////////////

// mount value through function
test_me_aligned!(test46;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_DOUBLE1",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_BREAK_DOUBLE1", "147", "mobel11000", "10981", "60", "20M80S", "", "", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["DOWNSTREAM_KEEP_BREAK_DOUBLE1", "99", "mobel11000", "10951", "60", "50M50S", "", "", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test47;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_DOUBLE1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "TTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
            coordinate: -49.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10951,
            left_clip: 0,
            right_boundry: 11050,
            rigth_clip: 50,
            signature: "50M50S".to_string(),
          },
          flag: 99,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "AAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
            coordinate: -79.,
          },
          cigar: CIGAR{
            align: vec![20],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10981,
            left_clip: 0,
            right_boundry: 11080,
            rigth_clip: 80,
            signature: "20M80S".to_string(),
          },
          flag: 147,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10981,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value through function
test_me_aligned!(test48;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_DOUBLE2",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_BREAK_DOUBLE2", "99", "mobel11000", "10951", "60", "50M50S", "", "", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["DOWNSTREAM_KEEP_BREAK_DOUBLE2", "147", "mobel11000", "10981", "60", "20M80S", "", "", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test49;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_DOUBLE2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "TTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
            coordinate: -49.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10951,
            left_clip: 0,
            right_boundry: 11050,
            rigth_clip: 50,
            signature: "50M50S".to_string(),
          },
          flag: 99,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "AAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
            coordinate: -79.,
          },
          cigar: CIGAR{
            align: vec![20],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10981,
            left_clip: 0,
            right_boundry: 11080,
            rigth_clip: 80,
            signature: "20M80S".to_string(),
          },
          flag: 147,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10981,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream multi anchor
// mount value manually
test_me_aligned!(test52;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_MULTI2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 1,
            left_clip: 0,
            right_boundry: 100,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 83,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 1,
          size: 11000.
        },
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![70],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 100,
            left_clip: 0,
            right_boundry: 199,
            rigth_clip: 30,
            signature: "70M30H".to_string(),
          },
          flag: 339,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 100,
          size: 11000.
        },
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![50],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 50,
            left_clip: 0,
            right_boundry: 149,
            rigth_clip: 50,
            signature: "50M50H".to_string(),
          },
          flag: 339,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 50,
          size: 11000.
        },
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![65],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 150,
            left_clip: 0,
            right_boundry: 249,
            rigth_clip: 35,
            signature: "65M35H".to_string(),
          },
          flag: 339,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 150,
          size: 11000.
        },
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 5000,
            left_clip: 0,
            right_boundry: 5099,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 339,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 5000,
          size: 11000.
        },
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 163,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 150,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream multi anchor
// mount value manually
test_me_aligned!(test57;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_MULTI1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10851,
            left_clip: 0,
            right_boundry: 10950,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 75,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10851,
          size: 11000.
        },
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![10],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10891,
            left_clip: 0,
            right_boundry: 10990,
            rigth_clip: 90,
            signature: "10M90H".to_string(),
          },
          flag: 331,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10891,
          size: 11000.
        },
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![40],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10801,
            left_clip: 0,
            right_boundry: 10900,
            rigth_clip: 60,
            signature: "40M60H".to_string(),
          },
          flag: 331,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10801,
          size: 11000.
        },
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![70],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 10831,
            left_clip: 0,
            right_boundry: 10930,
            rigth_clip: 30,
            signature: "70M30H".to_string(),
          },
          flag: 331,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Downstream,
          position: 10831,
          size: 11000.
        },
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 831,
            left_clip: 0,
            right_boundry: 930,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 331,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 831,
          size: 11000.
        },
      ],
      orientation: OrientationEnum::Downstream,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 135,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 10851,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// last record keep
// mount value through function
test_me_aligned!(test61;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "LAST_KEEP",
  Some(&chimeric_pair_build(
    &[
      &["LAST_KEEP", "91", "mobel11000", "101", "60", "100M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["LAST_KEEP", "167", "mobel11000", "101", "0", "*", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
test_me_aligned!(test62;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "LAST_KEEP",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.,
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 101,
            left_clip: 0,
            right_boundry: 200,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 91,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::Upstream,
          position: 101,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::Upstream,
      quality: 0,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.
          },
          cigar: CIGAR{
            align: vec![0],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 0,
            left_clip: 0,
            right_boundry: 0,
            rigth_clip: 0,
            signature: "*".to_string(),
          },
          flag: 167,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 101,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 0,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////
