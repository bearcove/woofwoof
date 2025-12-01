//! WOFF2 font compression and decompression.
//!
//! Uses the Google woff2 C++ library for font transforms,
//! with brotli compression provided by the pure Rust `brotli` crate.

mod brotli_shim;

mod ffi {
    use std::ffi::c_int;

    unsafe extern "C" {
        pub fn ComputeTTFToWOFF2Size(
            data: *const u8,
            length: usize,
            extended_metadata: *const i8,
            extended_metadata_length: usize,
        ) -> usize;

        pub fn ComputeWOFF2ToTTFSize(data: *const u8, length: usize) -> usize;

        pub fn ConvertTTFToWOFF2(
            data: *const u8,
            length: usize,
            result: *mut u8,
            result_length: *mut usize,
            extended_metadata: *const i8,
            extended_metadata_length: usize,
            brotli_quality: c_int,
            allow_transforms: c_int,
        ) -> c_int;

        pub fn ConvertWOFF2ToTTF(
            result: *mut u8,
            result_length: usize,
            data: *const u8,
            length: usize,
        ) -> c_int;
    }
}

/// Compress a TTF/OTF font to WOFF2 format.
///
/// # Arguments
/// * `data` - The TTF or OTF font data
/// * `metadata` - Optional extended metadata (XML)
/// * `quality` - Brotli compression quality (0-11, default 8)
/// * `transform` - Whether to apply font-specific transforms
///
/// # Returns
/// The WOFF2 compressed font data, or `None` if compression failed.
pub fn compress<T>(data: &[u8], metadata: T, quality: usize, transform: bool) -> Option<Vec<u8>>
where
    T: Into<Vec<u8>>,
{
    let metadata = match std::ffi::CString::new(metadata) {
        Ok(metadata) => metadata,
        _ => return None,
    };
    let metadata_size = metadata.count_bytes();

    let size = unsafe {
        ffi::ComputeTTFToWOFF2Size(
            data.as_ptr(),
            data.len(),
            metadata.as_ptr(),
            metadata_size,
        )
    };

    let mut buffer = vec![0u8; size];
    let mut size = buffer.len();

    let status = unsafe {
        ffi::ConvertTTFToWOFF2(
            data.as_ptr(),
            data.len(),
            buffer.as_mut_ptr(),
            &mut size,
            metadata.as_ptr(),
            metadata_size,
            quality as std::ffi::c_int,
            transform as std::ffi::c_int,
        )
    };

    if status == 0 {
        return None;
    }

    buffer.truncate(size);
    Some(buffer)
}

/// Decompress a WOFF2 font to TTF/OTF format.
///
/// # Arguments
/// * `data` - The WOFF2 compressed font data
///
/// # Returns
/// The decompressed TTF/OTF font data, or `None` if decompression failed.
pub fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let size = unsafe { ffi::ComputeWOFF2ToTTFSize(data.as_ptr(), data.len()) };

    if size == 0 {
        return None;
    }

    let mut buffer = vec![0u8; size];

    let status = unsafe {
        ffi::ConvertWOFF2ToTTF(buffer.as_mut_ptr(), size, data.as_ptr(), data.len())
    };

    if status == 0 {
        return None;
    }

    Some(buffer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let ttf = std::fs::read("tests/Roboto.ttf").expect("failed to read test font");

        // Compress
        let woff2 = compress(&ttf, "", 8, true).expect("compression failed");
        println!("TTF: {} bytes -> WOFF2: {} bytes", ttf.len(), woff2.len());

        // Verify WOFF2 signature
        assert_eq!(&woff2[0..4], b"wOF2", "invalid WOFF2 signature");

        // Decompress
        let roundtripped = decompress(&woff2).expect("decompression failed");

        // Note: roundtripped may not be byte-identical due to font transforms,
        // but should be a valid font of similar size
        println!("Decompressed: {} bytes", roundtripped.len());
        assert!(!roundtripped.is_empty());
    }
}
