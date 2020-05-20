
// standard libraries

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


fn effective_genome_length_calculator(genome_length: i32) -> i32 {
  let effective_genome_length = genome_length * BIN_SIZE / BIN_OVERLAP;
  return effective_genome_length
}


fn r_ppoisson(lambda: f64, psize: usize) -> Vec<f64> {
  let mut ppois_vec = vec![0.; psize];
  for ppois_index in 1..=psize {
    // fix lower_tail = TRUE & log_p = FALSE
    unsafe {
      ppois_vec[ppois_index - 1] = 1. - ppois(ppois_index as f64, lambda, 1, 0);
    }
  }
  return ppois_vec
}

pub fn thresholder(pop_reads: i32, genome_size: i32, ) {
  let eff_genome_length = effective_genome_length_calculator(genome_size);
  let lambda = pop_reads * BIN_SIZE / eff_genome_length;
  let p_values = r_ppoisson(lambda as f64, 20);

  let peak_prob = p_values * genome_size;

}

// // // R
// // eff_genome_length <- length_post_contig * bin_size / bin_overlaps
// //
// // pop_reads <- sum(peak_strand[, "seq_sum"])
// // lamm <- pop_reads * bin_size / eff_genome_length
// // lambda_ls[[which_strand]] <- lamm
// // p_values <- 1-ppois(1:(no_fdr), lambda = lamm)
// //
// // peak_prob <- p_values * length_post_contig
// // false_disc_values <- peak_prob / cumsum(table(peak_strand[, "seq_max"]))[1:no_fdr]
// // pop_thres_ls[[which_strand]] <- min(which(false_disc_values < false_discovery_tolerance))
