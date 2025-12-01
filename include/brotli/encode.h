/* Stub brotli encode header - actual implementation in Rust */
#ifndef BROTLI_ENC_ENCODE_H_
#define BROTLI_ENC_ENCODE_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

#define BROTLI_TRUE 1
#define BROTLI_FALSE 0
typedef int BROTLI_BOOL;

typedef enum BrotliEncoderMode {
    BROTLI_MODE_GENERIC = 0,
    BROTLI_MODE_TEXT = 1,
    BROTLI_MODE_FONT = 2
} BrotliEncoderMode;

#define BROTLI_DEFAULT_QUALITY 11
#define BROTLI_DEFAULT_WINDOW 22
#define BROTLI_DEFAULT_MODE BROTLI_MODE_GENERIC

/* Redirect to woofwoof-prefixed symbol to avoid conflicts */
#define BrotliEncoderCompress woofwoof_BrotliEncoderCompress

/* One-shot compression - implemented in Rust */
BROTLI_BOOL woofwoof_BrotliEncoderCompress(
    int quality,
    int lgwin,
    BrotliEncoderMode mode,
    size_t input_size,
    const uint8_t* input_buffer,
    size_t* encoded_size,
    uint8_t* encoded_buffer);

/* Helper to compute max compressed size */
static inline size_t BrotliEncoderMaxCompressedSize(size_t input_size) {
    /* Formula from brotli source: input + (input >> 14) + 11 + 2 */
    return input_size + (input_size >> 14) + 11 + 2;
}

#ifdef __cplusplus
}
#endif

#endif /* BROTLI_ENC_ENCODE_H_ */
