use swc::Compiler;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::napi_types::*;

#[cfg(target_arch = "wasm32")]
pub use crate::wasm_types::*;

#[cfg(not(target_arch = "wasm32"))]
pub use napi::Env;

#[cfg(target_arch = "wasm32")]
pub struct Env;

#[cfg(not(target_arch = "wasm32"))]
pub struct ExtraInfo<'a> {
  pub env: &'a Env,
  pub compiler: &'a Compiler,
}

#[cfg(target_arch = "wasm32")]
pub struct ExtraInfo;
