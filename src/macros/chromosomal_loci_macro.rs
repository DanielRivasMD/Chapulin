
////////////////////////////////////////////////////////////////////////////////////////////////////

macro_rules! cl_load {
  ($record: expr, $read_no: tt, $flines: expr) => {
    if ($record.$read_no.sequence == $flines[9]) || ($record.$read_no.sequence_reverser() == $flines[9]) {
      $record.$read_no.chr_read.push(ChrAnchor::loader(&$flines))
    }
  };
}

macro_rules! cl_mapq {
  ($record: expr, $read_no: tt) => {
    if $record.$read_no.chr_read.is_empty() || $record.$read_no.chr_read[0].mapq < MAPQ {
      true
    } else {
      false
    }
  };
}


////////////////////////////////////////////////////////////////////////////////////////////////////
