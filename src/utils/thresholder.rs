
// standard libraries
use std::collections::{HashMap};

// crate utilities
use crate::{
  settings::{
    constants::{
      BIN_SIZE,
      BIN_OVERLAP,
    },
  },
};

// C bindings
use libc::{
  c_int,
  c_double,
};

// R function signature declaration
#[link(name="Rmath")]
extern {
  fn ppois(
    x: c_double,
    lambda: c_double,
    lower_tail: c_int,
    log_p: c_int
  ) -> c_double;
}


fn effective_genome_length_calculator(
  genome_length: f64,
) -> f64 {
  let effective_genome_length = genome_length * BIN_SIZE as f64 / BIN_OVERLAP as f64;
  return effective_genome_length
}


fn r_ppoisson(
  lambda: f64,
  psize: usize,
) -> Vec<f64> {
  let mut ppois_vec = vec![0.; psize];
  for ppois_index in 1..=psize {
    // fix lower_tail = TRUE & log_p = FALSE
    unsafe {
      ppois_vec[ppois_index - 1] = 1. - ppois(ppois_index as f64, lambda, 1, 0);
    }
  }
  return ppois_vec
}

fn tabler(
  bined_hm: &HashMap<i32, Vec<String>>,
  psize: usize,
) -> Vec<f64> {
  let mut out_vec = vec![0.; psize];
  for (_, i) in bined_hm.iter() {
    let length_count = i.len();
    if length_count < psize {
      out_vec[length_count - 1] = out_vec[length_count - 1] + 1.;
    }
  }
  return out_vec
}

fn cumsum(
  mut cum_vec: Vec<f64>,
) -> Vec<f64> {
  let mut cumulus = 0.;
  for i in 0..cum_vec.len() {
    cumulus = cumulus + cum_vec[i];
    cum_vec[i] = cumulus;
  }
  return cum_vec
}

pub fn thresholder(
  pop_reads: f64,
  chromosome_size: f64,
  false_discovery_tolerance: f64,
  read_hm: &HashMap<i32, Vec<String>>,
  psize: usize,
) -> usize {
  let eff_genome_length = effective_genome_length_calculator(chromosome_size);
  let lambda = pop_reads * BIN_SIZE as f64 / eff_genome_length;
  let p_values = r_ppoisson(lambda, psize);

  let mut peak_prob = vec![0.; psize];
  for (ix, p_val) in p_values.iter().enumerate() {
    peak_prob[ix] = p_val * chromosome_size;
  }

  let bin_tb = tabler(read_hm, psize);
  let cum_bin_tb = cumsum(bin_tb);
  let mut false_disc_values = vec![0.; psize];
  for ix in 0..psize {
    false_disc_values[ix] = peak_prob[ix] / cum_bin_tb[ix];
  }

  let mut threshold = 0;
  for (ix, fd_val) in false_disc_values.iter().enumerate() {
    if *fd_val < false_discovery_tolerance {
      threshold = ix;
    }
  }
  return threshold;
}

// // R
// eff_genome_length <- length_post_contig * bin_size / bin_overlaps
//
// pop_reads <- sum(peak_strand[, "seq_sum"])
// lamm <- pop_reads * bin_size / eff_genome_length
// lambda_ls[[which_strand]] <- lamm
// p_values <- 1-ppois(1:(no_fdr), lambda = lamm)
//
// peak_prob <- p_values * length_post_contig
// false_disc_values <- peak_prob / cumsum(table(peak_strand[, "seq_max"]))[1:no_fdr]
// pop_thres_ls[[which_strand]] <- min(which(false_disc_values < false_discovery_tolerance))

////////////////////////////////////////////////////////////////////////////////////////////////////

// test private functions
#[cfg(test)]
mod tests {
  use data_test::data_test;
  use super::{ppois, r_ppoisson};

  data_test! {

    // test R ppois bindings
    fn test_ppois(sample, lambda, expected) => {
      let pois_vec: Vec<f64> = sample
        .iter()
        .map(|ix| unsafe {
            super::ppois(*ix, lambda, 1, 0)
          } )
        .collect();
      assert_eq!(pois_vec, expected)
    }

    // precission point => 20
    - zero (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 0., vec![1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, 1.0000000000000000000, ])
    - ichi (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 1., vec![0.36787944117144233402, 0.73575888234288466805, 0.91969860292860583506, 0.98101184312384615005, 0.99634015317265633982, 0.99940581518241833336, 0.99991675885071196195, 0.99998975080332530574, 0.99999887479740201535, 0.99999988857452171143, 0.99999998995223360332, ])
    - ni (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 2., vec![0.13533528323661270232, 0.40600584970983810695, 0.67667641618306340057, 0.85712346049854692964, 0.94734698265628891622, 0.98343639151938555543, 0.99546619447375117584, 0.99890328103214132138, 0.99976255267173885777, 0.99995350192498277941, 0.99999169177563151933, ])
    - san (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 3., vec![0.049787068367863944462, 0.19914827347145580561, 0.4231900811268435314, 0.64723188878223114617, 0.81526324452377196828, 0.91608205796869657256, 0.96649146469115876368, 0.98809549614364267089, 0.99619700793832399732, 0.99889751186988451348, 0.99970766304935265723, ])
    - yon (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 4., vec![0.018315638888734178669, 0.091578194443670934977, 0.23810330555354436433, 0.43347012036670901081, 0.62883693517987349075, 0.78513038703040516353, 0.8893260215974262417, 0.94886638420715263553, 0.97863656551201583245, 0.99186775720306608051, 0.99716023387948626855, ])
    - go (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 5., vec![0.0067379469990854670008, 0.040427681994512805475, 0.1246520194830811612, 0.26502591529736180265, 0.44049328506521229221, 0.61596065483306317034, 0.76218346297293870784, 0.86662832592999261561, 0.93190636527815151613, 0.96817194269379514004, 0.98630473140161711854, ])
    - roku (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 6., vec![0.0024787521766663584907, 0.017351265236664505098, 0.061968804416658973544, 0.15120388277664784105, 0.2850565003166312672, 0.44567964136461130087, 0.606302782412591168, 0.74397976045371705389, 0.84723749398456105197, 0.91607598300512427247, 0.95737907641746189391, ])
    - nana (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 7., vec![0.00091188196555451624375, 0.0072950557244361334194, 0.029636163880521784048, 0.08176541624472165315, 0.17299160788207135209, 0.30070827617436096668, 0.44971105584869880412, 0.59871383552303680808, 0.72909126773808230482, 0.83049593723867354278, 0.90147920588908725392, ])
    - hachi (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 8., vec![0.00033546262790251185322, 0.0030191636511226085764, 0.013753967744002993517, 0.042380111991684003836, 0.099632400487046052229, 0.19123606207962520753, 0.31337427753639757189, 0.45296080948699457558, 0.5925473414375912462, 0.71662425872701085439, 0.81588579255854631889, ])
    - ku (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 9., vec![0.00012340980408667956121, 0.0012340980408667970216, 0.0062321951063773160795, 0.021226486302908888215, 0.054963641495104915979, 0.11569052084105772848, 0.20678083985998707561, 0.32389696431289594081, 0.45565260432241877497, 0.58740824433194127607, 0.70598832034051173245, ])
    - juu (vec![0., 1., 2., 3., 4., 5., 6., 7., 8., 9., 10., ], 10., vec![4.5399929762484854173e-05, 0.00049939922738733333492, 0.0027693957155115757861, 0.010336050675925727987, 0.029252688076961082253, 0.067085962879031804662, 0.1301414208824830665, 0.22022064660169907158, 0.33281967875071877261, 0.45792971447185226719, 0.58303975019298537319, ])

  }
