use image::{ImageDecoder, ImageResult, ImageError};
use std::ptr;
//use libc;
use thiserror::Error;

/// Errors related to WSQ operations.
#[derive(Debug, Error)]
pub enum WsqError {
    #[error("WSQ compression failed with code: {0}")]
    CompressionFailed(i32),
    #[error("WSQ decompression failed with code: {0}")]
    DecompressionFailed(i32),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}


/// Public result type for WSQ operations.
pub type WsqResult<T> = Result<T, WsqError>;

// Extern definitions for the WSQ C API.
extern "C" {

    fn wsq_encode_mem(
        out_buffer: *mut *mut u8,
        out_buffer_size: *mut i32,
        r_bitrate: f32,
        raw_image: *const u8,
        width: i32,
        height: i32,
        depth: i32,
        ppi: i32,
        comment_text: *const u8,
    ) -> i32;

    fn wsq_decode_mem(
        out_buffer: *mut *mut u8,
        width: *mut i32,
        height: *mut i32,
        depth: *mut i32,
        ppi: *mut i32,
        lossy_flag: *mut i32,
        buffer: *const u8,
        buffer_size: i32,
    ) -> i32;
}

/// Compress a raw grayscale image into WSQ format.
///
/// # Parameters
/// - `raw_image`: The raw pixel data (grayscale, 8 bits per pixel).
/// - `width`: The width of the image.
/// - `height`: The height of the image.
/// - `bitrate`: The target bitrate for compression.
///
/// # Returns
/// A `Vec<u8>` containing the compressed WSQ data.
///
/// # Errors
/// Returns a `WsqError` if compression fails.
pub fn compress_wsq(
    raw_image: &[u8],
    width: i32,
    height: i32,
    bitrate: f32,
) -> WsqResult<Vec<u8>> {
    unsafe {
        let mut out_buffer: *mut u8 = ptr::null_mut();
        let mut out_buffer_size: i32 = 0;

        if width <= 0 || height <= 0 {
            return Err(WsqError::InvalidInput("Width and height must be positive.".to_string()));
        }
        
        let result = wsq_encode_mem(
            &mut out_buffer,
            &mut out_buffer_size,
            bitrate,
            raw_image.as_ptr(),
            width,
            height,
            1,  // Assume 1 byte per pixel depth
            -1, // Unknown PPI
            ptr::null(),
        );

            if result != 0 {
                return Err(WsqError::CompressionFailed(result));
        }

        let output = Vec::from_raw_parts(out_buffer, out_buffer_size as usize, out_buffer_size as usize);
        Ok(output)
    }
}

pub fn decompress_wsq(wsq_data: &[u8]) -> WsqResult<(Vec<u8>, i32, i32, i32)> {
    unsafe {
        let mut out_buffer: *mut u8 = ptr::null_mut();
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let mut depth: i32 = 0;
        let mut ppi: i32 = 0;
        let mut lossy_flag: i32 = 0;

        let result = wsq_decode_mem(
            &mut out_buffer,
            &mut width,
            &mut height,
            &mut depth,
            &mut ppi,
            &mut lossy_flag,
            wsq_data.as_ptr(),
            wsq_data.len() as i32,
        );

        if result != 0 {
            return Err(WsqError::DecompressionFailed(result));
        }
    
        if out_buffer.is_null() {
            return Err(WsqError::DecompressionFailed(-1));
        }

        let image_size = (width * height) as usize;
        let output = Vec::from_raw_parts(out_buffer, image_size, image_size);
        Ok((output, width, height, ppi))
    }
}

pub struct WsqDecoder {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl WsqDecoder {
    /// Create a new `WsqDecoder` by decompressing a WSQ image.
    pub fn new(data: Vec<u8>) -> WsqResult<Self> {
        let (decompressed_data, width, height, _ppi) = decompress_wsq(&data)?;
        Ok(Self {
            data: decompressed_data,
            width: width as u32,
            height: height as u32,
        })
    }
}

impl ImageDecoder for WsqDecoder {
    fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    fn color_type(&self) -> image::ColorType {
        image::ColorType::L8 // Assuming grayscale for WSQ
    }

    fn read_image(self, buf: &mut [u8]) -> ImageResult<()> {
        if buf.len() != self.data.len() {
            return Err(ImageError::Decoding(
                image::error::DecodingError::new(
                    image::error::ImageFormatHint::Unknown,
                    "Error in image buffer size.".to_string(),
                ),
            ));
        }
        buf.copy_from_slice(&self.data);
        Ok(())
    }

    fn read_image_boxed(self: Box<Self>, buf: &mut [u8]) -> ImageResult<()> {
        self.read_image(buf)
    }
}



/// Decode a WSQ file into a `WsqDecoder`.
///
/// # Parameters
/// - `path`: Path to the WSQ file.
///
/// # Returns
/// A `WsqDecoder` containing the decompressed image data.
///
/// # Errors
/// Returns a `WsqError` if file reading or decoding fails.
pub fn decode_wsq_file(path: &str) -> WsqResult<WsqDecoder> {
    // Read the WSQ file into a Vec<u8>
    let data = std::fs::read(path)?;
    WsqDecoder::new(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn decode_wsq() {
        let result = decode_wsq_file("test.wsq");
        assert!(result.is_ok());

        let decoded_wsq = result.unwrap();
        assert_eq!(decoded_wsq.width, 327);
        assert_eq!(decoded_wsq.height, 443);
        assert_eq!(decoded_wsq.data.len(), 144861);

        // println!(" decoded wsq -> w: {} h: {} len: {}", decoded_wsq.width, decoded_wsq.height, decoded_wsq.data.len());
    }

    #[test]
    fn decode_encode_wsq() {
        let result = decode_wsq_file("test.wsq");
        assert!(result.is_ok());
        let decoded_wsq = result.unwrap();
        
        // re-encode the same image
        let compress_result = compress_wsq(&decoded_wsq.data, decoded_wsq.width as i32, decoded_wsq.height as i32, 0.75);
        assert!(compress_result.is_ok());
        let encoded_wsq = compress_result.unwrap();
        
        //
        let decode_result = decompress_wsq(&encoded_wsq);
        assert!(decode_result.is_ok());
        let (decoded_buffer, w, h, _bpp) = decode_result.unwrap();

        assert_eq!(decoded_wsq.width, w as u32);
        assert_eq!(decoded_wsq.height, h as u32);
        assert_eq!(decoded_wsq.data.len(), decoded_buffer.len());

        // teste decoding from saved file... 
        fs::write("retest.wsq", encoded_wsq);
        let decode_file_result = decode_wsq_file("retest.wsq");
        assert!(decode_file_result.is_ok());
    }
}
