#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TxKit_Registry TxKit_Registry;

typedef struct {
    /**
     * pseudo-random seed
     */
    uint32_t global_seed;
    /**
     * lattice scale (size in pixels)
     */
    float scale;
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
