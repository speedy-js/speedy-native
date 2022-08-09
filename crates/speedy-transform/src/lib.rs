#[cfg(not(target_arch = "wasm32"))]
#[macro_use]
extern crate napi_derive;

/*
 * 引入测试用例 (编译时不会携带)
 */
mod test;

/*
 * 导出内容
 */
pub mod str;

#[cfg(not(target_arch = "wasm32"))]
pub mod napi_types;
#[cfg(target_arch = "wasm32")]
pub mod wasm_types;

pub mod web_transform;
