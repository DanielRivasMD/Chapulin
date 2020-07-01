
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
  genome_length: i32,
) -> i32 {
  let effective_genome_length = genome_length * BIN_SIZE / BIN_OVERLAP;
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
) -> Vec<i32> {
  let mut out_vec = vec![0; psize];
  for (_, i) in bined_hm.iter() {
    let length_count = i.len();
    if length_count < psize {
      out_vec[length_count - 1] = out_vec[length_count - 1] + 1;
    }
  }
  return out_vec
}

fn cumsum(
  mut cum_vec: Vec<i32>,
) -> Vec<i32> {
  let mut cumulus = 0;
  for i in 0..cum_vec.len() {
    cumulus = cumulus + cum_vec[i];
    cum_vec[i] = cumulus;
  }
  return cum_vec
}

pub fn thresholder(
  pop_reads: i32,
  genome_size: i32,
  false_discovery_tolerance: f64,
  read_hm: &HashMap<i32, Vec<String>>,
  psize: usize,
) -> i32 {
  let eff_genome_length = effective_genome_length_calculator(genome_size);
  let lambda = pop_reads * BIN_SIZE / eff_genome_length;
  let p_values = r_ppoisson(lambda as f64, psize);

  let mut peak_prob = vec![0.; psize];
  for (ix, p_val) in p_values.iter().enumerate() {
    peak_prob[ix] = p_val * genome_size as f64;
  }

  let bin_tb = tabler(read_hm, psize);
  let cum_bin_tb = cumsum(bin_tb);
  let mut false_disc_values = vec![0.; psize];
  for ix in 0..psize {
    false_disc_values[ix] = peak_prob[ix] / cum_bin_tb[ix] as f64;
  }

  let mut threshold = 0;
  for (ix, fd_val) in false_disc_values.iter().enumerate() {
    if *fd_val < false_discovery_tolerance {
      threshold = ix;
    }
  }
  return threshold as i32;
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
