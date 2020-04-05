
use std::collections::HashMap;

pub fn hashmap_init<T>() -> HashMap < String, T > {
  let out_hashmap: HashMap < String, T > = HashMap::new();
  return out_hashmap
}

