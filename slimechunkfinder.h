
#ifndef SLIME_CHUNK_FINDER_H
#define SLIME_CHUNK_FINDER_H

#ifdef __cplusplus
extern "C" {
#endif

#include<stdint.h>
#include<stdio.h>

#define FC "slimechunk.bin" //file name of cache
#define L 0x20000000 //bytes used by the cache

	uint32_t get_seed(int32_t, int32_t);
	uint8_t is_slime_chunk(uint32_t);
	void write_cache(int32_t, int32_t);
	uint8_t read_cache(uint32_t);

	uint8_t c[L] = { 0 };

#ifdef __cplusplus
}
#endif

#endif
