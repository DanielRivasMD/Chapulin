
// standard libraries
use rand_distr::{
  Poisson,
};
use crate::settings::constants::{BIN_SIZE, BIN_OVERLAP};

fn effective_genome_length_calculator(genome_length: i32) -> i32 {
  let effective_genome_length = genome_length * BIN_SIZE / BIN_OVERLAP;
  return effective_genome_length
}

pub fn thresholder(pop_reads: i32, genome_size: i32, ) {
  let eff_genome_length = effective_genome_length_calculator(genome_size);
  let lambda = pop_reads * BIN_SIZE / eff_genome_length;

  let x = Poisson::new(10.).unwrap();
  println!("{:?}", x);



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
