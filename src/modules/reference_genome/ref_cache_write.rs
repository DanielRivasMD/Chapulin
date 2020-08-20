
////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use std::collections::{HashMap};
use std::sync::{Arc, Mutex};
use anyhow::{Context};
use anyhow::Result as anyResult;
use std::fs::File as stdFile;
use std::io::{Write};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::{
  common_error::ChapulinCommonError,
};

////////////////////////////////////////////////////////////////////////////////////////////////////


pub fn write_cache(
  ref_cache: String,
  chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {

  let chr_hm = chr_assembly.lock().unwrap();
  let mut ref_cache_file = stdFile::create(&ref_cache)
    .context(ChapulinCommonError::CreateFile{ f: ref_cache })?;
    // .with_context(|| format!("{}: {}", ChapulinCommonError::CreateFile, &ref_cache))?;

  for (c, l) in chr_hm.iter() {
    let cl_write = format!("{}\t{}\n", c, l);
    ref_cache_file.write_all(cl_write.as_bytes())
      .unwrap();
      // .context(ChapulinCommonError::WriteFile)?;
  }

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
