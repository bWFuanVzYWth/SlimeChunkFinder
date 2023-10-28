#include "slimechunk.h"

#define BLOCK_SIZE 256
#define BLOCK_COUNT (4294967296 / BLOCK_SIZE)

#define N 624
#define M 397
#define MATRIX_A 0x9908b0dfUL
#define UPPER_MASK 0x80000000UL
#define LOWER_MASK 0x7fffffffUL

uint32_t to_seed(int32_t x, int32_t z) {
    return ((uint32_t)x * 0x1f1f1f1f) ^ (uint32_t)z;
}

uint32_t is_slime_chunk(uint32_t s) {
    uint32_t mt, y;

    mt = (1812433253UL * (s ^ (s >> 30)) + 1);

    y = (s & UPPER_MASK) | (mt & LOWER_MASK);

    for(uint32_t i = 2; i <= M; i++)
        mt = (1812433253UL * (mt ^ (mt >> 30)) + i);

    y = mt ^ (y >> 1) ^ ((-(y & 0x1UL)) & MATRIX_A);

    y ^= (y >> 11);
    y ^= (y << 7) & 0x9d2c5680UL;
    y ^= (y << 15) & 0xefc60000UL;
    y ^= (y >> 18);

    return !(y % 10);
}

void is_slime_chunk_simd(uint32_t* s) {
    uint32_t mt[BLOCK_SIZE];
    uint32_t y[BLOCK_SIZE];

    for(uint32_t n = 0; n < BLOCK_SIZE; n++) {
        mt[n] = (1812433253UL * (s[n] ^ (s[n] >> 30)) + 1);
        y[n] = (s[n] & UPPER_MASK) | (mt[n] & LOWER_MASK);
    }

    for(uint32_t i = 2; i <= M; i++) {
        for(uint32_t n = 0; n < BLOCK_SIZE; n++) {
            mt[n] = (1812433253UL * (mt[n] ^ (mt[n] >> 30)) + i);
        }
    }

    for(uint32_t n = 0; n < BLOCK_SIZE; n++) {
        y[n] = mt[n] ^ (y[n] >> 1) ^ ((-(y[n] & 0x1UL)) & MATRIX_A);

        y[n] ^= (y[n] >> 11);
        y[n] ^= (y[n] << 7) & 0x9d2c5680UL;
        y[n] ^= (y[n] << 15) & 0xefc60000UL;
        y[n] ^= (y[n] >> 18);

        s[n] = !(y[n] % 10);
    }
}

uint8_t is_slime_chunk_LUT(uint32_t seed, uint8_t* LUT) {
    return LUT[seed / 8] & (1 << (seed % 8));
}

uint8_t* generate_LUT(void) {
    uint8_t* LUT = calloc(536870912, 1);

#pragma omp parallel for
    for(uint32_t i = 0; i < BLOCK_COUNT; i++) {
        uint32_t block[BLOCK_SIZE];

        for(uint32_t j = 0; j < BLOCK_SIZE; j++)
            block[j] = i * BLOCK_SIZE + j;

        is_slime_chunk_simd(block);

        for(uint32_t j = 0; j < BLOCK_SIZE; j++) {
            uint32_t seed = i * BLOCK_SIZE + j;
            LUT[seed / 8] |= ((uint8_t)block[j]) << (seed % 8);
        }
    }

    return LUT;
}


