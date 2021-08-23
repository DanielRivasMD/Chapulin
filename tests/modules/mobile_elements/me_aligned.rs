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
  TagME,
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
macro_rules! me_aligned {
  ( $function: ident;
    mobile |> $mobel_id: expr, $mobel_size: expr;
    params |> $key: expr,  $val: expr;
  ) => {
    #[test]
    fn $function() {
      // declare files
      let me_alignment = "tests/samples/me_alignment.sam";

      // declare mobile element library
      let mutex_me_collection = alias::arc_map();

      // insert mobile element library
      mutex_me_collection
        .lock()
        .unwrap()
        .insert($mobel_id, $mobel_size);

      // declare chimeric mobile element collection
      let mutex_record_collection = alias::arc_map();
      let clone_mutex = alias::arc_clone(&mutex_record_collection);
      let _debug_mutex = alias::arc_clone(&mutex_record_collection);

      // identify mobile elements
      // observe that error is unwrap
      me_aligned::me_identificator(
        me_alignment,
        mutex_me_collection,
        mutex_record_collection,
        0,
      )
      .expect("Error occured at mobile element identificator!");

      // assert
      assert_eq!(clone_mutex.lock().unwrap().get($key), $val);
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
  chimeric_read.quality = raw_values.quality;
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

// no read id value
me_aligned!(test00;
  mobile |> "RANDOM_ME".to_string(), 1000.;
  params |> "RANDOM_ID", None;
);

// not available mobile element
me_aligned!(test01;
  mobile |> "MOBEL_NA".to_string(), 1000000.;
  params |> "MOBEL_NA", None;
);

// drop upstream
me_aligned!(test02;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_DROP", None;
);

// drop downstream
me_aligned!(test03;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_DROP", None;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream
// mount value through function
me_aligned!(test11;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP", "91", "mobel11000", "101", "60", "100M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["UPSTREAM_KEEP", "167", "mobel11000", "101", "0", "*", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
me_aligned!(test12;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0,
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
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0
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

// downstream
// mount value through function
me_aligned!(test16;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP", "75", "mobel11000", "10751", "60", "100M", "", "", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["DOWNSTREAM_KEEP", "135", "mobel11000", "10751", "0", "*", "", "", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
me_aligned!(test17;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0,
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
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0
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

// upstream break point anchor
// mount value through function
me_aligned!(test21;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_ANCHOR",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_BREAK_ANCHOR", "91", "mobel11000", "1", "60", "50S50M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["UPSTREAM_KEEP_BREAK_ANCHOR", "167", "mobel11000", "1", "0", "*", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
me_aligned!(test22;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_ANCHOR",
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
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTBANCHORAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0
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

// downstream break point anchor
// mount value through function
me_aligned!(test26;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_ANCHOR",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_BREAK_ANCHOR", "75", "mobel11000", "10951", "60", "50M50S", "=", "0", "100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
      &["DOWNSTREAM_KEEP_BREAK_ANCHOR", "135", "mobel11000", "10951", "0", "*", "=", "0", "-100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
    ],
    11000.,
  ));
);

// mount value manually
me_aligned!(test27;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_ANCHOR",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "TTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
            coordinate: -49.0,
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
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGANCHORBATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0
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

// upstream break point mate
// mount value through function
me_aligned!(test31;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_MATE",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_BREAK_MATE", "83", "mobel11000", "151", "60", "100M", "=", "0", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["UPSTREAM_KEEP_BREAK_MATE", "163", "mobel11000", "1", "60", "50S50M", "=", "0", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
me_aligned!(test32;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_BREAK_MATE",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0,
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
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCC".to_string(),
            coordinate: 50.0
          },
          // breakpoint: BreakPoint{
          //   sequence: "".to_string(),
          //   coordinate: 0.0
          // },
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
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACBANCHORGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream break point mate
// mount value through function
me_aligned!(test36;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_MATE",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_BREAK_MATE", "99", "mobel11000", "10851", "60", "100M", "", "", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["DOWNSTREAM_KEEP_BREAK_MATE", "147", "mobel11000", "10951", "60", "50M50S", "", "", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
me_aligned!(test37;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_BREAK_MATE",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0,
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
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0
          },
          // breakpoint: BreakPoint{
          //   sequence: "AGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
          //   coordinate: 50.0,
          // },
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
          orientation: OrientationEnum::None,
          position: 10951,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTANCHORBCAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream multi anchor
// mount value through function
me_aligned!(test51;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_MULTI",
  Some(&chimeric_pair_build(
    &[
      &["UPSTREAM_KEEP_MULTI", "83", "mobel11000", "1", "37", "100M", "=", "150", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["UPSTREAM_KEEP_MULTI", "163", "mobel11000", "150", "37", "100M", "=", "1", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
me_aligned!(test52;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP_MULTI",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0,
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
        }
      ],
      orientation: OrientationEnum::None,
      quality: 37,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 150,
            left_clip: 0,
            right_boundry: 249,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 163,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 150,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 37,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream multi anchor
// mount value through function
me_aligned!(test56;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_MULTI",
  Some(&chimeric_pair_build(
    &[
      &["DOWNSTREAM_KEEP_MULTI", "83", "mobel11000", "1", "37", "100M", "=", "150", "-100", "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN", "AAAAAEEDAAAAA????;A?A@AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
      &["DOWNSTREAM_KEEP_MULTI", "163", "mobel11000", "150", "37", "100M", "=", "1", "100", "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC", "BCCFFFFDHHHHH%2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"],
    ],
    11000.,
  ));
);

// mount value manually
me_aligned!(test57;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP_MULTI",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0,
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
        }
      ],
      orientation: OrientationEnum::None,
      quality: 37,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0
          },
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 150,
            left_clip: 0,
            right_boundry: 249,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          flag: 163,
          mobel: "mobel11000".to_string(),
          orientation: OrientationEnum::None,
          position: 150,
          size: 11000.
        }
      ],
      orientation: OrientationEnum::None,
      quality: 37,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// last record keep
// mount value through function
me_aligned!(test61;
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
me_aligned!(test62;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "LAST_KEEP",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0,
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
      orientation: OrientationEnum::None,
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![],
      me_read: vec![
        MEAnchor{
          breakpoint: BreakPoint{
            sequence: String::new(),
            coordinate: 0.0
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
