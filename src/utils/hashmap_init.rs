
// standard libraries
use std::collections::HashMap;

// generic hashmap init
pub fn hashmap_init<T>() -> HashMap<String, T> {
  let out_hashmap: HashMap<String, T> = HashMap::new();
  return out_hashmap
}

