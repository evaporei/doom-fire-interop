mod pixel_board;
mod random;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(not(target_arch = "wasm32"))]
mod standard_ffi;

/// iOS doesn't need a wrapper on `standard_ffi` module like Andriod does
/// so just declaring `standard_ffi` is enough for it.

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
mod android;
