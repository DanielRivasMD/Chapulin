////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::HashMap;

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  AnchorEnum,
  BinPosition,
  BreakPoint,
  ChrAnchor,
  ChrAnchorEnum,
  MEAnchor,
  MEChimericPair,
  MEChimericRead,
  OrientationEnum,
  StrandDirection,
  CIGAR,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// crate utilities
use crate::modules::{
  filter_cl,
  insert_me_library,
  load_cl_sam,
  load_me_sam,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// test chromosomal loci modules performance by testing `cl_mapper`
// observe that anchor registry is not tested but assumed to load
// since function returns no errors
macro_rules! test_cl_filter {
  ( $function: ident;
    mobile |> $mobel_id: expr, $mobel_size: expr;
    params |> $key: expr, $chr: expr, $val: expr;
  ) => {
    #[test]
    fn $function() {
      // insert mobile elment values onto arc clone
      let amx_me_library = insert_me_library($mobel_id, $mobel_size);

      // load mobile element sam & return arc clone
      let camx_me_record =
        load_me_sam("tests/samples/me_alignment.sam", amx_me_library);

      // load chromosomal loci sam & return arc clone
      let (camx_me_record_cl, camx_chr_registry) =
        load_cl_sam("tests/samples/cl_alignment.sam", camx_me_record);

      // filter chromosomal loci & return arc clone
      let (camx_me_record_as, _) =
        filter_cl($chr, camx_chr_registry, camx_me_record_cl);

      // assert
      assert_eq!(camx_me_record_as.lock().unwrap().get($key), $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// no value
test_cl_filter!(test00;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "RANDOM_ID", "", None;
);

test_cl_filter!(test02;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_DROP", "chrT", None;
);

test_cl_filter!(test03;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_DROP", "chrT", None;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// upstream keep
//////////////////////////////////////////////////

// mount value manually
test_cl_filter!(test12;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "UPSTREAM_KEEP1", "chrT",
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
  params |> "UPSTREAM_KEEP2", "chrN",
  None;
);

////////////////////////////////////////////////////////////////////////////////////////////////////

// downstream keep
//////////////////////////////////////////////////

// mount value manually
// MAPQ = 15
test_cl_filter!(test17;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP1", "chrT",
  None;
);

// mount value manually
test_cl_filter!(test19;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "DOWNSTREAM_KEEP2", "chrN",
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
          chr: "chrN".to_string(),
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
          chr: "chrN".to_string(),
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

macro_rules! test_cl_direction {
  ( $function: ident;
    mobile |> $mobel_id: expr, $mobel_size: expr;
    params |> $key: expr, $val: expr;
  ) => {
    #[test]
    fn $function() {
      // insert mobile elment values onto arc clone
      let amx_me_library = insert_me_library($mobel_id, $mobel_size);

      // load mobile element sam & return arc clone
      let camx_me_record =
        load_me_sam("tests/samples/me_alignment.sam", amx_me_library);

      // load chromosomal loci sam & return arc clone
      let (camx_me_record_cl, camx_chr_registry) =
        load_cl_sam("tests/samples/cl_alignment.sam", camx_me_record);

      // filter chromosomal loci & return arc clone
      let (_, camx_dir_registry_as) =
        filter_cl($key, camx_chr_registry, camx_me_record_cl);

      // assert
      assert_eq!(camx_dir_registry_as.lock().unwrap().get($key), $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

fn build_hash(
  k: i32,
  v: Vec<String>,
) -> HashMap<i32, Vec<String>> {
  let mut hm = HashMap::new();
  hm.insert(k, v);
  hm
}

////////////////////////////////////////////////////////////////////////////////////////////////////

test_cl_direction!(dir00;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "chrU", None;
);

test_cl_direction!(dir01;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "chrT",
  Some(&StrandDirection{
    fs5: BinPosition::new(),
    fs3: BinPosition::new(),
    rs5: BinPosition(1, build_hash(52500, vec!["UPSTREAM_KEEP1".to_string()])),
    rs3: BinPosition::new(),
  });
);

test_cl_direction!(dir02;
  mobile |> "mobel11000".to_string(), 11000.;
  params |> "chrN",
  Some(&StrandDirection{
    fs5: BinPosition::new(),
    fs3: BinPosition::new(),
    rs5: BinPosition::new(),
    rs3: BinPosition(1, build_hash(11700, vec!["DOWNSTREAM_KEEP2".to_string()])),
  });
);

////////////////////////////////////////////////////////////////////////////////////////////////////
