
// const SAM_FLAG_SIZE: usize = 12;

// 1) read paired
// 2) read mapped in proper pair
// 3) read unmapped
// 4) mate unmapped
// 5) read reverse strand
// 6) mate reverse strand
// 7) first in pair
// 8) second in pair
// 9) not primary alignment
// 10) read fails platform/vendor quality checks
// 11) read is PCR or optical duplicate
// 12) supplementary alignment

pub fn interpretor(n: i32, p: usize) -> bool {
  let bin_n = format!("{:b}", n);
  let mut stat_array = ['0'; 12];
  for i in bin_n.char_indices() {
    stat_array[i.0] = i.1;
  }
  // println!("{:#?}", stat_array);
  stat_array[p] == '1'
}
  // // let tmp_flag = flag_int(record_line[1].parse().unwrap());
  // let tmp_flag = format!("{:b}", record_line[1].parse::<i32>().unwrap());
  // let mut stat_string = ['0'; 12];
  // println!("{:#?}", stat_string);

  // for i in tmp_flag.char_indices() {
  // stat_string[i.0] = i.1;
  // // println!("{:?}", i);
  // }

  // println!("{:#?}", stat_string);
  // println!("{:#?}", tmp_flag);



// pub fn flag_int(n: i32) -> [i32; SAM_FLAG_SIZE] {
//   let a = binary_int(n);
//   let b = number_to_vec(a);
//   return b
// }
//
// pub fn binary_int(n: i32) -> i32 {
//   let m = format!("{:b}", n);
//   let k = m.parse::<i32>().unwrap();
//   return k
// }
//
// pub fn number_to_vec(n: i32) -> [i32; SAM_FLAG_SIZE] {
//   let mut digits = [0; SAM_FLAG_SIZE];
//   let mut n = n;
//   let mut c = 0;
//   while n > 9 {
//     digits[c] = (n % 10);
//     n = n / 10;
//     println!("{}", n);
//     c = c + 1;
//   }
//   // digits.reverse();
//   return digits
// }
