/**
 * @file noise.glsl
 * @brief Shared noise definitions
 * @author Alixinne <alixinne@pm.me>
 */

#ifndef _NOISE_GLSL_
#define _NOISE_GLSL_

#ifndef PARAM_GLOBAL_SEED
#define PARAM_GLOBAL_SEED globalSeed
layout(location = 10) uniform uint globalSeed;
#endif /* PARAM_GLOBAL_SEED */

#endif /* _NOISE_GLSL_ */

// vim: ft=glsl.doxygen
