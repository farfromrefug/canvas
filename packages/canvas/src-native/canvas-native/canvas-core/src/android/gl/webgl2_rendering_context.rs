#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::os::raw::c_void;

use jni::objects::{JByteBuffer, JClass, JObject, ReleaseMode};
use jni::sys::{
    jboolean, jbyteArray, jdouble, jdoubleArray, jfloatArray, jint, jintArray, jlong, jlongArray,
    jshortArray, JNI_TRUE,
};
use jni::JNIEnv;

use crate::common::context::image_asset::ImageAsset;

const RGBA: u32 = 0x1908;
const RGBA_INTEGER: u32 = 0x8D99;

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeFlipInPlace3D(
    env: JNIEnv,
    _: JClass,
    pixels: JByteBuffer,
    bytesPerPixel: jint,
    height: jint,
    depth: jint,
) {
    if let Ok(buf) = env.get_direct_buffer_address(pixels) {
        crate::common::utils::gl::flip_in_place_3d(
            buf.as_mut_ptr(),
            buf.len(),
            bytesPerPixel as usize,
            height as usize,
            depth as usize,
        );
    }
}

fn texImage3D(
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    flipY: bool,
    buf: &mut [u8],
) {
    if flipY {
        crate::common::utils::gl::flip_in_place_3d(
            buf.as_mut_ptr(),
            buf.len(),
            crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                * width as usize,
            height as usize,
            depth as usize,
        );
    }
    unsafe {
        gl_bindings::glTexImage3D(
            target as u32,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format as u32,
            image_type as u32,
            buf.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DBuffer(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    buffer: JByteBuffer,
    flipY: jboolean,
) {
    if let Ok(data_array) = env.get_direct_buffer_address(buffer) {
        texImage3D(
            target,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format,
            image_type,
            flipY == JNI_TRUE,
            data_array,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DByteArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    byteArray: jbyteArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(byteArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(array.as_ptr() as *mut u8, size);
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texImage3D: byte get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DShortArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    shortArray: jshortArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(shortArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i16>(),
            );
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texImage3D: short get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DIntArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    intArray: jintArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(intArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i32>(),
            );
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!("texImage3D: int get_primitive_array_critical error {:?}", e);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DFloatArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    floatArray: jfloatArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(floatArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<f32>(),
            );
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texImage3D: float get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DDoubleArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    doubleArray: jdoubleArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(doubleArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<f64>(),
            );
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texImage3D: double get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DLongArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    longArray: jlongArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(longArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i64>(),
            );
            texImage3D(
                target,
                level,
                internalformat,
                width,
                height,
                depth,
                border,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texImage3D: long get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DAsset(
    _env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    asset: jlong,
    flipY: jboolean,
) {
    let asset: *mut ImageAsset = asset as _;
    let asset = &mut *asset;
    let mut data;
    match format as u32 {
        RGBA | RGBA_INTEGER => data = asset.rgba_internal_bytes(),
        _ => data = asset.rgb_internal_bytes(),
    }
    let data_array = data.as_mut_slice();
    if flipY == JNI_TRUE {
        crate::common::utils::gl::flip_in_place_3d(
            data_array.as_mut_ptr(),
            data_array.len(),
            crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                * asset.width() as usize,
            asset.height() as usize,
            depth as usize,
        );
    }
    gl_bindings::glTexImage3D(
        target as u32,
        level,
        internalformat,
        width,
        height,
        depth,
        border,
        format as u32,
        image_type as u32,
        data_array.as_ptr() as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexImage3DBitmap(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    internalformat: jint,
    width: jint,
    height: jint,
    depth: jint,
    border: jint,
    format: jint,
    image_type: jint,
    bitmap: JObject,
    flipY: jboolean,
) {
    let mut data = super::super::utils::image::get_bytes_from_bitmap(env, bitmap);
    if !data.0.is_empty() {
        if flipY == JNI_TRUE {
            crate::common::utils::gl::flip_in_place_3d(
                data.0.as_mut_ptr(),
                data.0.len(),
                crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                    as usize
                    * data.1.width as usize,
                data.1.height as usize,
                depth as usize,
            );
        }
        gl_bindings::glTexImage3D(
            target as u32,
            level,
            internalformat,
            width,
            height,
            depth,
            border,
            format as u32,
            image_type as u32,
            data.0.as_ptr() as *const c_void,
        );
    }
}

fn texSubImage3D(
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    flipY: bool,
    buf: &mut [u8],
) {
    if flipY {
        crate::common::utils::gl::flip_in_place_3d(
            buf.as_mut_ptr(),
            buf.len(),
            crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                * width as usize,
            height as usize,
            depth as usize,
        );
    }
    unsafe {
        gl_bindings::glTexSubImage3D(
            target as u32,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format as u32,
            image_type as u32,
            buf.as_ptr() as *const c_void,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DBuffer(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    buffer: JByteBuffer,
    flipY: jboolean,
) {
    if let Ok(data_array) = env.get_direct_buffer_address(buffer) {
        texSubImage3D(
            target,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format,
            image_type,
            flipY == JNI_TRUE,
            data_array,
        );
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DByteArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    byteArray: jbyteArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(byteArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(array.as_ptr() as *mut u8, size);
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage3D: byte get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DShortArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    shortArray: jshortArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(shortArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i16>(),
            );
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage3D: short get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DIntArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    intArray: jintArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(intArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i32>(),
            );
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage3D: short get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DLongArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    longArray: jlongArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(longArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<i64>(),
            );
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage3D: long get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DFloatArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    floatArray: jfloatArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(floatArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<f32>(),
            );
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage3D: float  get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DDoubleArray(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    doubleArray: jdoubleArray,
    flipY: jboolean,
) {
    match env.get_primitive_array_critical(doubleArray, ReleaseMode::NoCopyBack) {
        Ok(array) => {
            let size = array.size().unwrap_or(0) as usize;
            let buf = std::slice::from_raw_parts_mut(
                array.as_ptr() as *mut u8,
                size * std::mem::size_of::<f64>(),
            );
            texSubImage3D(
                target,
                level,
                xoffset,
                yoffset,
                zoffset,
                width,
                height,
                depth,
                format,
                image_type,
                flipY == JNI_TRUE,
                buf,
            );
        }
        Err(e) => {
            log::debug!(
                "texSubImage3D: double get_primitive_array_critical error {:?}",
                e
            );
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DAsset(
    _env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    asset: jlong,
    flipY: jboolean,
) {
    let asset: *mut ImageAsset = asset as _;
    let asset = &mut *asset;
    let mut data;
    match format as u32 {
        RGBA | RGBA_INTEGER => data = asset.rgba_internal_bytes(),
        _ => data = asset.rgb_internal_bytes(),
    }
    let data_array = data.as_mut_slice();
    if flipY == JNI_TRUE {
        crate::common::utils::gl::flip_in_place_3d(
            data_array.as_mut_ptr(),
            data_array.len(),
            crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32) as usize
                * asset.width() as usize,
            asset.height() as usize,
            depth as usize,
        );
    }
    gl_bindings::glTexSubImage3D(
        target as u32,
        level,
        xoffset,
        yoffset,
        zoffset,
        width,
        height,
        depth,
        format as u32,
        image_type as u32,
        data_array.as_ptr() as *const c_void,
    );
}

#[no_mangle]
pub unsafe extern "C" fn Java_org_nativescript_canvas_TNSWebGLRenderingContext_nativeTexSubImage3DBitmap(
    env: JNIEnv,
    _: JClass,
    target: jint,
    level: jint,
    xoffset: jint,
    yoffset: jint,
    zoffset: jint,
    width: jint,
    height: jint,
    depth: jint,
    format: jint,
    image_type: jint,
    bitmap: JObject,
    flipY: jboolean,
) {
    let mut data = super::super::utils::image::get_bytes_from_bitmap(env, bitmap);
    if !data.0.is_empty() {
        if flipY == JNI_TRUE {
            crate::common::utils::gl::flip_in_place_3d(
                data.0.as_mut_ptr(),
                data.0.len(),
                crate::common::utils::gl::bytes_per_pixel(image_type as u32, format as u32)
                    as usize
                    * data.1.width as usize,
                data.1.height as usize,
                depth as usize,
            );
        }
        gl_bindings::glTexSubImage3D(
            target as u32,
            level,
            xoffset,
            yoffset,
            zoffset,
            width,
            height,
            depth,
            format as u32,
            image_type as u32,
            data.0.as_ptr() as *const c_void,
        );
    }
}
