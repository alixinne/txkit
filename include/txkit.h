#ifndef TXKIT_H
#define TXKIT_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * No error occurred
 */
#define TXKIT_SUCCESS 0

/**
 * Type of elements in an image
 */
enum ImageDataType
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
  /**
   * Unsigned bytes (8 bits)
   */
  UInt8,
  /**
   * Single-precision floating point (32 bits)
   */
  Float32,
};
#ifndef __cplusplus
typedef uint32_t ImageDataType;
#endif // __cplusplus

/**
 * txkit computing context
 */
typedef struct Context Context;

/**
 * Image that can be sent accross for FFI
 */
typedef struct Image Image;

/**
 * Wrapped method for FFI
 */
typedef struct MethodBox MethodBox;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void txkit_context_destroy(Context *ctx);

Context *txkit_context_new_cpu(void);

Context *txkit_context_new_gpu(void);

/**
 * Get the description of the last error that occurred in the txkit API
 *
 * # Returns
 *
 * Null pointer if no error occurred, or error message for the last error.
 */
const int8_t *txkit_get_last_error(void);

/**
 * Return the number of channels of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
uint32_t txkit_image_channels(const Image *image);

/**
 * Return a pointer to the image data
 *
 * # Parameters
 *
 * * `image`: target image
 */
const void *txkit_image_data(const Image *image);

/**
 * Return the depth (Z size) of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
uint32_t txkit_image_depth(const Image *image);

/**
 * Destroy an image
 *
 * # Parameters
 *
 * * `image`: image to destroy
 */
void txkit_image_destroy(Image *image);

/**
 * Return the element type of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
ImageDataType txkit_image_element_type(const Image *image);

/**
 * Return the height (Y size) of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
uint32_t txkit_image_height(const Image *image);

/**
 * Create a new unsigned byte image
 *
 * # Parameters
 *
 * * `width`: width of the image
 * * `height`: height of the image
 * * `channels`: number of channels in the image
 *
 * # Returns
 *
 */
Image *txkit_image_new_f32(uintptr_t width, uintptr_t height, uintptr_t channels);

/**
 * Create a new unsigned byte image
 *
 * # Parameters
 *
 * * `width`: width of the image
 * * `height`: height of the image
 * * `channels`: number of channels in the image
 *
 * # Returns
 *
 * Allocated image.
 */
Image *txkit_image_new_u8(uintptr_t width, uintptr_t height, uintptr_t channels);

/**
 * Return the width (X size) of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
uint32_t txkit_image_width(const Image *image);

/**
 * Compute an image using the given method
 *
 * # Parameters
 *
 * * `ctx`: context to use for computing the image
 * * `method`: texturing method
 * * `tgt`: target image to be computed
 *
 * # Returns
 *
 * TXKIT_SUCCESS if no error occurred, else a non-zero code.
 */
int32_t txkit_method_compute(Context *ctx, MethodBox *method, Image *tgt);

/**
 * Destroy a method
 */
void txkit_method_destroy(MethodBox *method);

/**
 * Create a new method by name
 *
 * # Parameters
 *
 * * `method_name`: name of the method to create
 *
 * # Returns
 *
 * Null pointer if an error occurred creating the method, otherwise pointer to the allocated
 * method.
 */
MethodBox *txkit_method_new(const uint8_t *method_name);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* TXKIT_H */
