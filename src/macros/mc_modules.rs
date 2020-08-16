
////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! load {

  // mobile element
  ( $record: expr, $read_no: tt, $flines: expr, $ms: expr, $mo: expr ) => {
    if $flines[1].parse::<i32>().unwrap() <= 255 {
      $record.$read_no.sequence = $flines[9].to_string();
    }
    $record.$read_no.me_read.push(MEAnchor::loader(&$flines, $ms, &$mo));
  };

  // structural variant
  ( $record: expr, $read_no: tt, $flines: expr ) => {
    $record.$read_no.sequence = $flines[9].to_string();
    $record.$read_no.chr_read = ChrAnchor::loader(&$flines);
  }

}

macro_rules! reload {

  // chromosomal loci
  ( $record: expr, $read_no: tt, $flines: expr ) => {
    if ( $record.$read_no.sequence == $flines[9] ) || ( $record.$read_no.sequence_reverser() == $flines[9] ) {
      $record.$read_no.chr_read.push(ChrAnchor::loader(&$flines))
    }
  };

}

macro_rules! mapq {

  // mapq
  ( $record: expr, $read_no: tt ) => {
    $record.$read_no.chr_read.is_empty() || $record.$read_no.chr_read[0].mapq < MAPQ
  };

}

////////////////////////////////////////////////////////////////////////////////////////////////////
