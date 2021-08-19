////////////////////////////////////////////////////////////////////////////////////////////////////

// standard library
use std::sync::Arc;

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
      .unwrap();

      // dbg!(_debug_mutex.lock().unwrap().get($key));

      // assert
      assert_eq!(clone_mutex.lock().unwrap().get($key), $val);
    }
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

// TODO: update sample file. values below are not kept by algorithm
// tests

// no value
me_aligned!(test00; assert_eq; "Random_ID"; None; ""; 0.);

// // mount value through function
// me_aligned!(test02; assert_eq; "CS0001.1";
//   Some(&chimeric_pair_build(&[
//     &["CS0001.1", "75", "REF_cs100", "1", "37", "100M", "=", "150", "-100",
// "AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN"
// , "AAAAAEEDAAAAA????;A?A@
// AAADDDDDDDIIIIIIDIIIIEEIIICIIIECIEEEDIIIIIDEIIEIIIIIIIIIIIIIIIEEBDDD:
// DDDB=B=1%", "NM:i:5  MD:Z:19G2A4T18C0C10 MC:Z:73S27M AS:i:33 XS:i:33"],
//     &["CS0001.1", "135", "REF_cs100", "150", "37", "100M", "=", "1", "100",
// "TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC"
// , "BCCFFFFDHHHHH%
// 2AEGIIIIIIIIIIIIIIIIIIIGH<FHIIIGIIIIIIGGI=FCGIIIIIHHCHFFFDAD@A>;
// ACDDDDB>CFFED?CDDDDDCC", "NM:i:2  MD:Z:19G2A4 MC:Z:33S58M9S AS:i:19 XS:i:19"]
//     ]));
// "REF_cs100";
// 1000.
// );

// // mount value manually
// me_aligned!(test03; assert_eq; "CS0001.1";
//   Some(&MEChimericPair{
//     read1: MEChimericRead{
//       breakpoint: BreakPoint{
//         sequence: String::new(),
//         coordinate: 0,
//       },
//       chr_read: vec![],
//       me_read: vec![
//         MEAnchor{
//           cigar: CIGAR{
//             align: vec![100],
//             del: vec![],
//             ins: vec![],
//             lclip: 0,
//             left_boundry: 1,
//             rclip: 0,
//             right_boundry: 101,
//             signature: String::from("100M"),
//           },
//           flag: 75,
//           mobel: String::from("REF_cs100"),
//           orientation: OrientationEnum::None,
//           position: 1,
//           size: 1000.0
//         }
//       ],
//       quality: 37,
//       sequence:
// String::from("
// AGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGACAGGGTTTCACCATGTTGGTTAGGCTGGTCTCAAACTCCTN"
// ),     },
//     read2: MEChimericRead{
//       breakpoint: BreakPoint{
//         sequence: String::new(),
//         coordinate: 0
//       },
//       chr_read: vec![],
//       me_read: vec![
//         MEAnchor{
//           cigar: CIGAR{
//             align: vec![100],
//             del: vec![],
//             ins: vec![],
//             lclip: 0,
//             left_boundry: 150,
//             rclip: 0,
//             right_boundry: 250,
//             signature: String::from("100M"),
//           },
//           flag: 135,
//           mobel: String::from("REF_cs100"),
//           orientation: OrientationEnum::None,
//           position: 150,
//           size: 1000.0
//         }
//       ],
//       quality: 37,
//       sequence:
// String::from("
// TCCAGGGTTCAAGNGATTCTCCTGCCTCAGCCTCCAGAGTAGCTGAGACTACAGGTGTCCGCCACCAGGCCCAGCTAATTTTTGTATTTTTATTAGAGAC"
// ),     },
//     chranch: ChrAnchorEnum::None,
//   });
// "REF_cs100";
// 1000.
// );

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

  // FIX: HARDCODED anchor tag. write tag method on mobile element chimeric pair
  chimeric_pair.chranch = ChrAnchorEnum::Read2;

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

  // mobile element
  chimeric_read.me_read.push(MEAnchor::load(
    raw_values.cigar.clone(),
    raw_values.flag,
    raw_values.scaffold.clone(),
    raw_values.orientation.clone(),
    raw_values.position,
    raw_values.get_extra(),
  ));

  // tag mobile element anchors iteratively
  chimeric_read
    .me_read
    .iter_mut()
    .for_each(|me_anchor| me_anchor.tag());

  chimeric_read.quality = raw_values.quality;
  chimeric_read.sequence = raw_values.sequence;

  chimeric_read
}

////////////////////////////////////////////////////////////////////////////////////////////////////
