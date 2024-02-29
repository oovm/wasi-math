pub use crate::vm::MathRuntime;
pub use crate::host::MathContext;

wasmtime::component::bindgen!({
    path: "../wit",
});

mod host;
mod vm;
