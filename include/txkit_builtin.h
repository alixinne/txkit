#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TxKit_Registry TxKit_Registry;

/**
 * A 2-dimensional vector.
 *
 * This type is marked as `#[repr(C)]`.
 */
typedef struct {
    /**
     * The x component of the vector.
     */
    float x;
    /**
     * The y component of the vector.
     */
    float y;
} TxKit_Vector2_f32;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
    /**
     * lattice scale (size in pixels)
     */
    float scale;
    /**
     * stats mode (0: normal, 1: process, 2: lookat)
     */
    int32_t stats_mode;
    /**
     * look-at parameter (if stats_mode == lookat) in [0, 1]^2
     */
    TxKit_Vector2_f32 stats_look_at;
} TxKit_GradientNoiseParams;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
    /**
     * lattice scale (size in pixels)
     */
    float scale;
    /**
     * stats mode (0: normal, 1: process, 2: lookat)
     */
    int32_t stats_mode;
    /**
     * look-at parameter (if stats_mode == lookat) in [0, 1]^2
     */
    TxKit_Vector2_f32 stats_look_at;
} TxKit_ValueNoiseParams;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
} TxKit_WhiteNoiseParams;

typedef struct {
    float alpha_value;
} TxKit_DebugParams;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Create a new registry with txkit built-in methods registered
 *
 * # Returns
 *
 * Pointer to the allocated registry.
 */
TXKIT_API TxKit_Registry *txkit_registry_new_builtin(void);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus
