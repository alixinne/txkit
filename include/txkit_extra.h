#ifndef TXKIT_EXTRA_H
#define TXKIT_EXTRA_H

#if defined _WIN32 || defined __CYGWIN__
  #ifdef __GNUC__
    #define TXKIT_API __attribute__ ((dllimport))
  #else
    #define TXKIT_API __declspec(dllimport)
  #endif
#else
  #if __GNUC__ >= 4
    #define TXKIT_API __attribute__ ((visibility ("default")))
  #else
    #define TXKIT_API
  #endif
#endif

#endif /* TXKIT_EXTRA_H */
