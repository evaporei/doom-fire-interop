#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(js_namespace = Math)]
    pub fn random() -> f64;

}

#[cfg(not(target_arch = "wasm32"))]
use rand::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn random() -> f64 {
    rand::thread_rng().gen()
}

#[cfg(all(
    not(target_arch = "wasm32"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
pub fn random() -> f64 {
    unimplemented!();
}
