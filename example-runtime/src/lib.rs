pub use crate::vm::MathRuntime;
pub use crate::host::MathContext;
pub use num::{BigInt, bigint::Sign};

wasmtime::component::bindgen!({
     path: "../wit",
     with: {
          "wasi:math/types/integer": BigInt,
          "wasi:math/types/integer-buffer": BigInt,
     },
});



mod host;
mod vm;
