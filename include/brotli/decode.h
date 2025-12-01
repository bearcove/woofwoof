/* Stub brotli decode header - actual implementation in Rust */
#ifndef BROTLI_DEC_DECODE_H_
#define BROTLI_DEC_DECODE_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef enum {
    BROTLI_DECODER_RESULT_ERROR = 0,
    BROTLI_DECODER_RESULT_SUCCESS = 1,
    BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT = 2,
    BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT = 3
} BrotliDecoderResult;

/* Redirect to woofwoof-prefixed symbol to avoid conflicts */
#define BrotliDecoderDecompress woofwoof_BrotliDecoderDecompress

/* One-shot decompression - implemented in Rust */
BrotliDecoderResult woofwoof_BrotliDecoderDecompress(
    size_t encoded_size,
    const uint8_t* encoded_buffer,
    size_t* decoded_size,
    uint8_t* decoded_buffer);

#ifdef __cplusplus
}
#endif

#endif /* BROTLI_DEC_DECODE_H_ */
