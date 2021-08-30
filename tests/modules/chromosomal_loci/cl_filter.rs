////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  AnchorEnum,
  BreakPoint,
  ChrAnchor,
  ChrAnchorEnum,
  MEAnchor,
  MEChimericPair,
  MEChimericRead,
  OrientationEnum,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// aliases
use chapulin::utils::alias;

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use chapulin::modules::{
  chromosomal_loci::cl_aligned,
  chromosomal_loci::cl_filter,
  mobile_elements::me_aligned,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// test chromosomal loci modules performance by testing `cl_mapper`
// observe that anchor registry is not tested but assumed to load
// since function returns no errors
macro_rules! test_cl_filter {
  ( $function: ident;
    mobile |> $mobel_id: expr, $mobel_size: expr;
    params |> $key: expr, $val: expr;
  ) => {
    #[test]
    fn $function() {
      // declare files
      let me_alignment = "tests/samples/me_alignment.sam";
      let cl_alignment = "tests/samples/cl_alignment.sam";

      // declare mobile element library
      let amx_me_library = alias::arc_map();

      // insert mobile element library
      amx_me_library
        .lock()
        .unwrap()
        .insert($mobel_id, $mobel_size);

      // declare record collection
      let amx_me_record = alias::arc_map();

      // declare chimeric mobile element clone
      let camx_me_record_me = alias::arc_clone(&amx_me_record);

      // identify mobile elements
      me_aligned::me_identificator(
        me_alignment,
        amx_me_library,
        camx_me_record_me,
        0,
      )
      .expect("Error occured at mobile element identificator!");

      // declare anchor registry
      let amx_anchor_registry = alias::arc_map();

      // declare anchor registry aligned clone
      let camx_anchor_registry_aligned = alias::arc_clone(&amx_anchor_registry);

      // declare chimeric chromosomal loci clone
      let camx_me_record_cl_aligned = alias::arc_clone(&amx_me_record);

      // map chromosomal loci
      cl_aligned::cl_mapper(
        cl_alignment,
        camx_anchor_registry_aligned,
        camx_me_record_cl_aligned,
        0,
      )
      .expect("Error occured at chromosomal loci mapper!");

      // declare anchor registry clone
      let camx_anchor_registry_filter = alias::arc_clone(&amx_anchor_registry);

      // declare direction registry clone
      let amx_dir_registry = alias::arc_map();

      // declare chimeric chromosomal loci filter clone
      let camx_me_record_cl_filter = alias::arc_clone(&amx_me_record);

      // filter chromosomal loci
      cl_filter::filter(
        "chrT",
        &camx_anchor_registry_filter,
        &amx_dir_registry,
        &camx_me_record_cl_filter,
      );

      // declare assertion clone
      let camx_me_record_as = alias::arc_clone(&amx_me_record);

      // assert
      assert_eq!(camx_me_record_as.lock().unwrap().get($key), $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// no value
test_cl_filter!(test00;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "RANDOM_ID", None;
);

test_cl_filter!(test02;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_DROP", None;
);

test_cl_filter!(test03;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_DROP", None;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream keep
//////////////////////////////////////////////////

// mount value manually
test_cl_filter!(test12;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP1",
  Some(&MEChimericPair{
    read2: MEChimericRead{
      chr_read: vec![
        ChrAnchor{
          anchor: AnchorEnum::None,
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 52137,
            left_clip: 0,
            right_boundry: 52236,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          chr: "chrT".to_string(),
          flag: 99,
          mapq: 60,
          position: 52137,
          tlen: -100,
        }
      ],
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
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read1: MEChimericRead{
      chr_read: vec![
        ChrAnchor{
          anchor: AnchorEnum::None,
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 52549,
            left_clip: 0,
            right_boundry: 52648,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          chr: "chrT".to_string(),
          flag: 147,
          mapq: 20,
          position: 52549,
          tlen: 100,
        }
      ],
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
      quality: 20,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read1,
  });
);

// mount value manually
// MAPQ = 10
test_cl_filter!(test14;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP2",
  None;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream keep
//////////////////////////////////////////////////

// mount value manually
// MAPQ = 15
test_cl_filter!(test17;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP1",
  None;
);

// mount value manually
test_cl_filter!(test19;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP2",
  Some(&MEChimericPair{
    read1: MEChimericRead{
      chr_read: vec![
        ChrAnchor{
          anchor: AnchorEnum::None,
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
          chr: "chrT".to_string(),
          flag: 147,
          mapq: 60,
          position: 10751,
          tlen: -100,
        }
      ],
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
      quality: 60,
      sequence: "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN".to_string(),
    },
    read2: MEChimericRead{
      chr_read: vec![
        ChrAnchor{
          anchor: AnchorEnum::None,
          cigar: CIGAR{
            align: vec![100],
            deletion: vec![],
            insertion: vec![],
            left_boundry: 11751,
            left_clip: 0,
            right_boundry: 11850,
            rigth_clip: 0,
            signature: "100M".to_string(),
          },
          chr: "chrT".to_string(),
          flag: 99,
          mapq: 50,
          position: 11751,
          tlen: 100,
        }
      ],
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
      quality: 50,
      sequence: "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC".to_string(),
    },
    chranch: ChrAnchorEnum::Read2,
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////
