////////////////////////////////////////////////////////////////////////////////////////////////////

/// calculate effective genome length with default parameters: BIN_SIZE & BIN_OVERLAP
macro_rules! effective_genome_length_calculator {
  // explicit values
  ( $gl: expr, $bs: expr, $bo: expr ) => {
    effective_genome_length_calculator($gl, $bs, $bo)
  };

  // default values
  ( $gl: expr ) => {
    effective_genome_length_calculator($gl, BIN_SIZE as f64, BIN_OVERLAP as f64)
  };
}

/// calculate poisson's lambda default parameters: BIN_SIZE
macro_rules! lambda_calculator {
  // explicit values
  ( $pr: expr, $egl: expr, $bs: expr ) => {
    lambda_calculator($pr, $egl, $bs)
  };

  // default values
  ( $pr: expr, $egl: expr ) => {
    lambda_calculator($pr, $egl, BIN_SIZE as f64)
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! chr_counter {
  // mobile element chromosomal counter
  ( $read_id: expr, $read_count: expr, $chr_pair: expr, $position_hm: expr ) => {
    let binned_position = format!("{}", $chr_pair.binner());
    chr_counter($read_id, $position_hm, binned_position);
    $read_count += 1;
  };

  // structural variant chromosomal counter
  ( $read_id: expr, $sv_pair: expr, $position_hm: expr ) => {
    let binned_pos = (
      $sv_pair.read1.chr_read.binner(),
      $sv_pair.read2.chr_read.binner(),
    );
    let binned_position = format!(
      "{}-{}",
      std::cmp::min(binned_pos.0, binned_pos.1),
      std::cmp::max(binned_pos.0, binned_pos.1)
    );
    chr_counter($read_id, $position_hm, binned_position);
  };
}

////////////////////////////////////////////////////////////////////////////////////////////////////
