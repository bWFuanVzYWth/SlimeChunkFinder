#include <stdint.h>
#include <stdlib.h>
#include <omp.h>

uint32_t to_seed(int32_t x, int32_t z);
uint32_t is_slime_chunk(uint32_t s);
void is_slime_chunk_simd(uint32_t* s);
uint8_t is_slime_chunk_LUT(uint32_t seed, uint8_t* LUT);
uint8_t* generate_LUT(void);
