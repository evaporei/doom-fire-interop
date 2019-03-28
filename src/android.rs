use jni::objects::{JObject, JValue};
use jni::sys::{jbyteArray, jint, jlong};
use jni::JNIEnv;

use crate::pixel_board::PixelBoard;
use crate::standard_ffi;

#[no_mangle]
pub extern "C" fn Java_com_otaviopace_doomfireandroid_DoomFire_createBoard(
    _env: JNIEnv,
    _this: JObject,
    width: jint,
    height: jint,
) -> jlong {
    standard_ffi::create_board(width as usize, height as usize) as jlong
}

#[no_mangle]
pub extern "C" fn Java_com_otaviopace_doomfireandroid_DoomFire_createFireSource(
    _env: JNIEnv,
    _this: JObject,
    board_ptr: jlong,
) {
    standard_ffi::create_fire_source(board_ptr as *mut PixelBoard);
}

#[no_mangle]
pub extern "C" fn Java_com_otaviopace_doomfireandroid_DoomFire_getPixels(
    env: JNIEnv,
    _this: JObject,
    board_ptr: jlong,
) -> jbyteArray {
    let pixel_board = unsafe { &*(board_ptr as *const PixelBoard) };

    let pixels = pixel_board.get_pixels();

    let pixels = unsafe { &*(pixels as *const [u8] as *const [i8]) };

    let byte_array = env.new_byte_array(pixels.len() as i32).unwrap();

    env.set_byte_array_region(byte_array, 0, pixels).unwrap();

    byte_array
}

#[no_mangle]
pub extern "C" fn Java_com_otaviopace_doomfireandroid_DoomFire_calculateFirePropagation(
    env: JNIEnv<'static>,
    this: JObject<'static>,
    board_ptr: jlong,
) {
    let pixel_board = unsafe { &mut *(board_ptr as *mut PixelBoard) };

    pixel_board.calculate_fire_propagation(Box::new(move |pixels| {
        let mut pixels = &mut pixels.to_vec();

        let byte_buffer = env.new_direct_byte_buffer(&mut pixels).unwrap();

        env.call_method(
            this,
            "render",
            "(Ljava/nio/ByteBuffer;)V",
            &[JValue::from(JObject::from(byte_buffer))],
        )
        .unwrap();
    }));
}

#[no_mangle]
pub extern "C" fn Java_com_otaviopace_doomfireandroid_DoomFire_freeBoard(
    _env: JNIEnv,
    _: JObject,
    board_ptr: jlong,
) {
    standard_ffi::free_board(board_ptr as *mut PixelBoard);
}
