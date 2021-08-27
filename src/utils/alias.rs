////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Result as anyhowResult;
use bytelines::ByteLines;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// development libraries
use genomic_structures::{
  MEChimericPair,
  SVChimericPair,
  StrandDirection,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// anyhow
pub type AnyResult = anyhowResult<()>;
pub type AnyBufferResult = anyhowResult<ByteLines<BufReader<File>>>;

////////////////////////////////////////////////////////////////////////////////////////////////////

// Mobile element
pub type LibraryME = Arc<Mutex<HashMap<String, f64>>>;
// pub type LibraryME = Arc<Mutex<HashMap<String, MElibrary>>>;
pub type RecordME = Arc<Mutex<HashMap<String, MEChimericPair>>>;
pub type RegistryChr = Arc<Mutex<HashMap<String, Vec<String>>>>;
pub type RegistryDir = Arc<Mutex<HashMap<String, StrandDirection>>>;

////////////////////////////////////////////////////////////////////////////////////////////////////

// Structural variant
pub type RecordSV = Arc<Mutex<HashMap<String, SVChimericPair>>>;

////////////////////////////////////////////////////////////////////////////////////////////////////

// Arc wrapper
pub fn arc_map<T, U>() -> Arc<Mutex<HashMap<T, U>>> {
  Arc::new(Mutex::new(HashMap::new()))
}

pub fn arc_clone<T, U>(
  arc_ref: &Arc<Mutex<HashMap<T, U>>>
) -> Arc<Mutex<HashMap<T, U>>> {
  Arc::clone(arc_ref)
}

////////////////////////////////////////////////////////////////////////////////////////////////////
