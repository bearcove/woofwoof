//! C ABI shim for brotli, allowing the woff2 C++ library to call into Rust brotli.

use std::ffi::c_int;
use std::slice;

use brotli::enc::backward_references::BrotliEncoderMode;
use brotli::enc::BrotliEncoderParams;
use brotli::BrotliCompress;
use brotli::BrotliDecompress;

// Constants matching brotli C API
pub const BROTLI_TRUE: c_int = 1;
pub const BROTLI_FALSE: c_int = 0;

// BrotliDecoderResult values
pub const BROTLI_DECODER_RESULT_ERROR: c_int = 0;
pub const BROTLI_DECODER_RESULT_SUCCESS: c_int = 1;

// BrotliEncoderMode values (must match C enum)
const BROTLI_MODE_TEXT: c_int = 1;
const BROTLI_MODE_FONT: c_int = 2;

/// Compress data using brotli.
///
/// This matches the C API signature:
/// ```c
/// BROTLI_BOOL BrotliEncoderCompress(
///     int quality, int lgwin, BrotliEncoderMode mode,
///     size_t input_size, const uint8_t* input_buffer,
///     size_t* encoded_size, uint8_t* encoded_buffer);
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BrotliEncoderCompress(
    quality: c_int,
    lgwin: c_int,
    mode: c_int,
    input_size: usize,
    input_buffer: *const u8,
    encoded_size: *mut usize,
    encoded_buffer: *mut u8,
) -> c_int {
    if input_buffer.is_null() || encoded_size.is_null() || encoded_buffer.is_null() {
        return BROTLI_FALSE;
    }

    unsafe {
        let input = slice::from_raw_parts(input_buffer, input_size);
        let max_output_size = *encoded_size;
        let output = slice::from_raw_parts_mut(encoded_buffer, max_output_size);

        let params = BrotliEncoderParams {
            quality,
            lgwin,
            mode: match mode {
                BROTLI_MODE_TEXT => BrotliEncoderMode::BROTLI_MODE_TEXT,
                BROTLI_MODE_FONT => BrotliEncoderMode::BROTLI_MODE_FONT,
                _ => BrotliEncoderMode::BROTLI_MODE_GENERIC,
            },
            ..Default::default()
        };

        let mut input_cursor = std::io::Cursor::new(input);
        let mut output_cursor = std::io::Cursor::new(output);

        match BrotliCompress(&mut input_cursor, &mut output_cursor, &params) {
            Ok(_) => {
                *encoded_size = output_cursor.position() as usize;
                BROTLI_TRUE
            }
            Err(_) => BROTLI_FALSE,
        }
    }
}

/// Decompress brotli-compressed data.
///
/// This matches the C API signature:
/// ```c
/// BrotliDecoderResult BrotliDecoderDecompress(
///     size_t encoded_size, const uint8_t* encoded_buffer,
///     size_t* decoded_size, uint8_t* decoded_buffer);
/// ```
#[unsafe(no_mangle)]
pub unsafe extern "C" fn BrotliDecoderDecompress(
    encoded_size: usize,
    encoded_buffer: *const u8,
    decoded_size: *mut usize,
    decoded_buffer: *mut u8,
) -> c_int {
    if encoded_buffer.is_null() || decoded_size.is_null() || decoded_buffer.is_null() {
        return BROTLI_DECODER_RESULT_ERROR;
    }

    unsafe {
        let input = slice::from_raw_parts(encoded_buffer, encoded_size);
        let max_output_size = *decoded_size;
        let output = slice::from_raw_parts_mut(decoded_buffer, max_output_size);

        let mut input_cursor = std::io::Cursor::new(input);
        let mut output_cursor = std::io::Cursor::new(output);

        match BrotliDecompress(&mut input_cursor, &mut output_cursor) {
            Ok(_) => {
                *decoded_size = output_cursor.position() as usize;
                BROTLI_DECODER_RESULT_SUCCESS
            }
            Err(_) => BROTLI_DECODER_RESULT_ERROR,
        }
    }
}
