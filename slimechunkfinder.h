#ifndef SLIMECHUNKFINDER
#define SLIMECHUNKFINDER

#include <time.h>
#include <stdint.h>
#include <stdio.h>

#define FC "slimechunk.bin" //file name of cache
#define L 0x20000000		//bytes used by the cache

#define MAX_W 0x80000		//max valid chunk position
#define MAX_N 16            //MAX_N>=13

void slime_map(int, int, int, int);
void slime_finder(int, int, int, int, int, int);
int slime_initialization(void);

void swap(int*, int*, int*, int*);

uint32_t get_seed(int32_t, int32_t);
uint8_t is_slime_chunk(uint32_t);
void write_cache(uint32_t, uint32_t);
uint8_t read_cache(uint32_t);

#endif