#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * No error occurred
 */
#define TxKit_SUCCESS 0

/**
 * Type of elements in an image
 */
enum TxKit_ImageDataType
#ifdef __cplusplus
  : uint32_t
#endif // __cplusplus
 {
    /**
     * Unsigned bytes (8 bits)
     */
    TxKit_ImageDataType_UInt8,
    /**
     * Single-precision floating point (32 bits)
     */
    TxKit_ImageDataType_Float32,
};
#ifndef __cplusplus
typedef uint32_t TxKit_ImageDataType;
#endif // __cplusplus

/**
 * txkit computing context
 */
typedef struct TxKit_Context TxKit_Context;

/**
 * Image that can be sent accross for FFI
 */
typedef struct TxKit_Image TxKit_Image;

typedef struct TxKit_MappedImageDataRead TxKit_MappedImageDataRead;

typedef struct TxKit_MappedImageDataWrite TxKit_MappedImageDataWrite;

/**
 * Wrapped method for FFI
 */
typedef struct TxKit_Method TxKit_Method;

typedef struct {
    uintptr_t width;
    uintptr_t height;
    uintptr_t depth;
    uintptr_t channels;
} TxKit_ImageDimensions_usize;

typedef TxKit_ImageDimensions_usize TxKit_ImageDim;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

TXKIT_API void txkit_context_destroy(TxKit_Context *ctx);

TXKIT_API TxKit_Context *txkit_context_new_cpu(void);

TXKIT_API TxKit_Context *txkit_context_new_gpu(void);

/**
 * Get the description of the last error that occurred in the txkit API
 *
 * # Returns
 *
 * Null pointer if no error occurred, or error message for the last error.
 */
TXKIT_API const char *txkit_get_last_error(void);

/**
 * Destroy an image
 *
 * # Parameters
 *
 * * `image`: image to destroy
 */
TXKIT_API void txkit_image_destroy(TxKit_Image *image);

/**
 * Return the dimensions of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
TXKIT_API TxKit_ImageDim txkit_image_dim(const TxKit_Image *image);

/**
 * Return the element type of the image
 *
 * # Parameters
 *
 * * `image`: target image
 */
TXKIT_API TxKit_ImageDataType txkit_image_element_type(const TxKit_Image *image);

/**
 * Map the image pixels for read access. The image must be unmapped after being used.
 *
 * # Parameters
 *
 * * `image`: image to map for read access
 */
TXKIT_API TxKit_MappedImageDataRead *txkit_image_map_read(const TxKit_Image *image);

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
TXKIT_API const float *txkit_image_map_read_data_f32(const TxKit_MappedImageDataRead *read_map);

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
TXKIT_API const uint8_t *txkit_image_map_read_data_u8(const TxKit_MappedImageDataRead *read_map);

/**
 * Map the image pixels for write access. The image must be unmapped after being used.
 *
 * # Parameters
 *
 * * `image`: image to map for write access
 */
TXKIT_API TxKit_MappedImageDataWrite *txkit_image_map_write(TxKit_Image *image);

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
TXKIT_API float *txkit_image_map_write_data_f32(TxKit_MappedImageDataWrite *write_map);

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
TXKIT_API uint8_t *txkit_image_map_write_data_u8(TxKit_MappedImageDataWrite *write_map);

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
TXKIT_API TxKit_Image *txkit_image_new_cpu(TxKit_ImageDim dim, TxKit_ImageDataType element_type);

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
TXKIT_API
TxKit_Image *txkit_image_new_gpu_1d(TxKit_ImageDim dim,
                                    TxKit_ImageDataType element_type,
                                    const TxKit_Context *context);

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
TXKIT_API
TxKit_Image *txkit_image_new_gpu_2d(TxKit_ImageDim dim,
                                    TxKit_ImageDataType element_type,
                                    const TxKit_Context *context);

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
TXKIT_API
TxKit_Image *txkit_image_new_gpu_3d(TxKit_ImageDim dim,
                                    TxKit_ImageDataType element_type,
                                    const TxKit_Context *context);

/**
 * Sync the host representation of the image with its device counterpart
 *
 * # Parameters
 *
 * * `image`: image to sync
 */
TXKIT_API int32_t txkit_image_sync(TxKit_Image *image);

/**
 * Unmap a mapped image.
 *
 * # Parameters
 *
 * * `read_map`: mapped image object
 */
TXKIT_API void txkit_image_unmap_read(TxKit_MappedImageDataRead *read_map);

/**
 * Unmap a mapped image.
 *
 * # Parameters
 *
 * * `write_map`: mapped image object
 */
TXKIT_API void txkit_image_unmap_write(TxKit_MappedImageDataWrite *write_map);

/**
 * Compute an image using the given method
 *
 * # Parameters
 *
 * * `ctx`: context to use for computing the image
 * * `method`: texturing method
 * * `tgt`: target image to be computed
 * * `params`: pointer to the parameter structure for this method
 * * `params_size`: size of the parameter structure
 *
 * # Returns
 *
 * TxKit_SUCCESS if no error occurred, else a non-zero code.
 */
TXKIT_API
int32_t txkit_method_compute(TxKit_Context *ctx,
                             TxKit_Method *method,
                             TxKit_Image *tgt,
                             const void *params,
                             uintptr_t params_size);

/**
 * Destroy a method
 */
TXKIT_API void txkit_method_destroy(TxKit_Method *method);

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
TXKIT_API TxKit_Method *txkit_method_new(const char *method_name);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus