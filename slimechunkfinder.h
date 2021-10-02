#ifndef _SLIMECHUNKFINDER
#define _SLIMECHUNKFINDER

#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>

#include <time.h>
#include <omp.h>

#define LUT_NAME "slimechunk.bin"
#define LUT_SIZE 0x20000000

typedef uint_fast8_t fast_t;

#define FAST_SIZE sizeof(uint_fast8_t)
#define FAST_BITS (FAST_SIZE * 8)
#define LUT_LEN (LUT_SIZE / FAST_SIZE)

#define MAX_W 0x100000
#define MAX_N 16

void swap(int *, int *);

void generate_lut(void);

fast_t is_slime_chunk_mt(uint32_t);
fast_t is_slime_chunk_lut(uint32_t);
uint32_t get_seed(int32_t, int32_t);
int slime_initialization(void);
void slime_map(int, int, int, int);
void slime_finder(int, int, int, int, int, int);

#endif