#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct TxKit_Registry TxKit_Registry;

typedef struct {

} TxKit_GradientNoiseParams;

typedef struct {

} TxKit_ValueNoiseParams;

typedef struct {

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
