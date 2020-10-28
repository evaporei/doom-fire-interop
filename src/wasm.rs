use cfg_if::cfg_if;
use js_sys::{Function, Uint8Array};
use wasm_bindgen::prelude::*;

// auto generated code for wee_alloc
cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// auto generated code for console_error_panic_hook
cfg_if! {
    if #[cfg(feature = "console_error_panic_hook")] {
        pub use console_error_panic_hook::set_once as set_panic_hook;
    } else {
        #[inline]
        pub fn set_panic_hook() {}
    }
}

fn to_uint8_array(slice: &[u8]) -> Uint8Array {
    let u_int8_array = Uint8Array::new_with_length(slice.len() as u32);

    for (position, item) in slice.iter().enumerate() {
        u_int8_array.fill(*item, position as u32, (position + 1) as u32);
    }

    u_int8_array
}

fn convert_render_callback(render: Function) -> Box<dyn Fn(&[u8])> {
    Box::new(move |pixels: &[u8]| {
        render
            .call1(&JsValue::NULL, &to_uint8_array(pixels))
            .expect("Error calling JS `render` function");
    })
}

use crate::pixel_board::PixelBoard;

// Public interface

#[wasm_bindgen]
pub fn create_board(fire_width: usize, fire_height: usize) -> PixelBoard {
    PixelBoard::new(fire_width, fire_height)
}

#[wasm_bindgen]
pub fn create_fire_source(board: &mut PixelBoard) {
    board.create_fire_source();
}

#[wasm_bindgen]
pub fn calculate_fire_propagation(board: &mut PixelBoard, render_callback: Function) {
    board.calculate_fire_propagation(convert_render_callback(render_callback));
}
