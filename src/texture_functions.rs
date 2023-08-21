extern crate gl;

use std::{path::Path, ffi::c_void};


extern crate image;
use image::GenericImage;


pub unsafe fn new_texture(texture_path: &str, display_type: i32, flipv: bool, fliph: bool) -> u32 {
    let mut texture = 0;

    gl::GenTextures(1, &mut texture);
    gl::BindTexture(gl::TEXTURE_2D, texture);


    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, display_type);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, display_type);

    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    let mut img = image::open(&Path::new(texture_path)).expect("Failed to load texture");

    if flipv {
        img = img.flipv();
    }

    if fliph {
        img = img.fliph();
    }

    let data = img.raw_pixels();

    gl::TexImage2D(gl::TEXTURE_2D,
                   0,
                   gl::RGBA as i32,
                   img.width() as i32,
                   img.height() as i32,
                   0,
                   gl::RGB,
                   gl::UNSIGNED_BYTE,
                   &data[0] as *const u8 as *const c_void);
    gl::GenerateMipmap(gl::TEXTURE_2D);

    texture
}
