////////////////////////////////////////////////////////////////////////////////////////////////////

// standard libraries
use anyhow::Context;
use anyhow::Result as anyResult;
use std::collections::HashMap;
use std::fs::File as stdFile;
use std::io::Write;
use std::sync::{
  Arc,
  Mutex,
};

////////////////////////////////////////////////////////////////////////////////////////////////////

// error handler
use crate::error::common_error::ChapulinCommonError;

////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn write_cache(
  ref_cache: &str,
  chr_assembly: Arc<Mutex<HashMap<String, f64>>>,
) -> anyResult<()> {
  let chr_hm = chr_assembly.lock().unwrap();
  let mut ref_cache_file = stdFile::create(ref_cache).context(ChapulinCommonError::CreateFile {
    f: ref_cache.to_string(),
  })?;

  for (c, l) in chr_hm.iter() {
    let cl_write = format!("{}\t{}\n", c, l);
    ref_cache_file
      .write_all(cl_write.as_bytes())
      .context(ChapulinCommonError::WriteFile {
        f: cl_write
      })?;
  }

  info!("Writing cache: {}", ref_cache);

  Ok(())
}

////////////////////////////////////////////////////////////////////////////////////////////////////
