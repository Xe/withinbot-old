#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
  int32_t base;
  int32_t prices[14];
} prices;

prices stonks(int32_t pattern, int32_t seed);
