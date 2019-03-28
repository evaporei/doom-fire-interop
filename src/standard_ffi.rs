use crate::pixel_board::PixelBoard;
use std::ptr;

fn convert_render(render: extern "C" fn(&[u8], usize)) -> Box<Fn(&[u8])> {
    Box::new(move |pixels: &[u8]| {
        render(pixels, pixels.len());
    })
}

#[no_mangle]
pub extern "C" fn create_board(fire_width: usize, fire_height: usize) -> *mut PixelBoard {
    let pixel_board = PixelBoard::new(fire_width, fire_height);

    let boxed_pixel_board = Box::new(pixel_board);

    Box::into_raw(boxed_pixel_board)
}

#[no_mangle]
pub extern "C" fn create_fire_source(pixel_board: *mut PixelBoard) {
    if pixel_board.is_null() {
        return;
    }

    let pixel_board = unsafe { &mut *pixel_board };
    pixel_board.create_fire_source();
}

#[no_mangle]
pub extern "C" fn get_pixels_ptr(board: *const PixelBoard) -> *const usize {
    if board.is_null() {
        return ptr::null();
    }

    let pixel_board = unsafe { &*board };
    pixel_board.get_pixels_ptr()
}

#[no_mangle]
pub extern "C" fn get_pixels_len(board: *const PixelBoard) -> usize {
    if board.is_null() {
        return 0;
    }

    let pixel_board = unsafe { &*board };
    pixel_board.get_pixels_len()
}

#[no_mangle]
pub extern "C" fn calculate_fire_propagation(
    pixel_board: *mut PixelBoard,
    render_callback: extern "C" fn(&[u8], usize),
) {
    if pixel_board.is_null() {
        return;
    }

    let pixel_board = unsafe { &mut *pixel_board };
    pixel_board.calculate_fire_propagation(convert_render(render_callback));
}

#[no_mangle]
pub extern "C" fn free_board(pixel_board: *mut PixelBoard) {
    unsafe {
        Box::from_raw(pixel_board);
    }
}
