
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
  stat_array[p] == '1'
}
