
////////////////////////////////////////////////////////////////////////////////////////////////////



// TODO: macro to produce defaults
// Used as an associated item.
macro_rules! const_maker {
    ($t:ty, $v:tt) => { const CONST: $t = $v; };
}
trait X {
    // const_maker!{i32, 7}
  const_maker!{i32, 200}
}

trait Y {
    // const_maker!{i32, 7}
  const_maker!{i32, 100}
}

trait Z {
    // const_maker!{i32, 7}
  const_maker!{i32, 400}
}

////////////////////////////////////////////////////////////////////////////////////////////////////
