struct ImageDim {
  uintptr_t width;
  uintptr_t height;
  uintptr_t depth;
  uintptr_t channels;
};

#ifndef __cplusplus
typedef struct ImageDim ImageDim;
#endif // __cplusplus

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
