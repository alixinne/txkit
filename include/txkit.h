#ifndef TXKIT_H
#define TXKIT_H

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "txkit_types.h"

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

typedef struct MappedImageDataRead MappedImageDataRead;

typedef struct MappedImageDataWrite MappedImageDataWrite;

/**
 * Wrapped method for FFI
 */
typedef struct Method Method;

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
 * Destroy an image
 *
 * # Parameters
 *
 * * `image`: image to destroy
 */
void txkit_image_destroy(Image *image);

/**
 * Return the dimensions of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
ImageDim txkit_image_dim(const Image *image);

/**
 * Return the element type of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
ImageDataType txkit_image_element_type(const Image *image);

/**
 * Map the image pixels for read access. The image must be unmapped after being used.
 *
 * # Parameters
 *
 * * `image`: image to map for read access
 */
MappedImageDataRead *txkit_image_map_read(const Image *image);

/**
 * Get a pointer to the image pixels through the given map.
 *
 * # Parameters
 *
 * * `read_map`: map to access
 *
 * # Returns
 *
 * Pointer to the pixel data, or null if the conversion failed.
 */
const float *txkit_image_map_read_data_f32(const MappedImageDataRead *read_map);

/**
 * Get a pointer to the image pixels through the given map.
 *
 * # Parameters
 *
 * * `read_map`: map to access
 *
 * # Returns
 *
 * Pointer to the pixel data, or null if the conversion failed.
 */
const uint8_t *txkit_image_map_read_data_u8(const MappedImageDataRead *read_map);

/**
 * Map the image pixels for write access. The image must be unmapped after being used.
 *
 * # Parameters
 *
 * * `image`: image to map for write access
 */
MappedImageDataWrite *txkit_image_map_write(Image *image);

/**
 * Get a pointer to the image pixels through the given map.
 *
 * # Parameters
 *
 * * `write_map`: map to access
 *
 * # Returns
 *
 * Pointer to the pixel data, or null if the conversion failed.
 */
float *txkit_image_map_write_data_f32(MappedImageDataWrite *write_map);

/**
 * Get a pointer to the image pixels through the given map.
 *
 * # Parameters
 *
 * * `write_map`: map to access
 *
 * # Returns
 *
 * Pointer to the pixel data, or null if the conversion failed.
 */
uint8_t *txkit_image_map_write_data_u8(MappedImageDataWrite *write_map);

/**
 * Create a new image for CPU-based computations
 *
 * # Parameters
 *
 * * `dim`: dimensions of the image
 * * `element_type`: type of the elements in the image
 *
 * # Returns
 *
 * Allocated image.
 */
Image *txkit_image_new_cpu(ImageDim dim, ImageDataType element_type);

/**
 * Create a new 1D image for GPU-based computations
 *
 * # Parameters
 *
 * * `dim`: dimensions of the image
 * * `element_type`: type of the elements in the image
 *
 * # Returns
 *
 * Allocated image.
 */
Image *txkit_image_new_gpu_1d(ImageDim dim, ImageDataType element_type, const Context *context);

/**
 * Create a new 2D image for GPU-based computations
 *
 * # Parameters
 *
 * * `dim`: dimensions of the image
 * * `element_type`: type of the elements in the image
 *
 * # Returns
 *
 * Allocated image.
 */
Image *txkit_image_new_gpu_2d(ImageDim dim, ImageDataType element_type, const Context *context);

/**
 * Create a new 3D image for GPU-based computations
 *
 * # Parameters
 *
 * * `dim`: dimensions of the image
 * * `element_type`: type of the elements in the image
 *
 * # Returns
 *
 * Allocated image.
 */
Image *txkit_image_new_gpu_3d(ImageDim dim, ImageDataType element_type, const Context *context);

/**
 * Unmap a mapped image.
 *
 * # Parameters
 *
 * * `read_map`: mapped image object
 */
void txkit_image_unmap_read(MappedImageDataRead *read_map);

/**
 * Unmap a mapped image.
 *
 * # Parameters
 *
 * * `write_map`: mapped image object
 */
void txkit_image_unmap_write(MappedImageDataWrite *write_map);

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
int32_t txkit_method_compute(Context *ctx, Method *method, Image *tgt);

/**
 * Destroy a method
 */
void txkit_method_destroy(Method *method);

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
Method *txkit_method_new(const char *method_name);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* TXKIT_H */
