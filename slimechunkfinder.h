
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

	uint8_t is_slime_chunk(uint32_t s)
	{
		uint32_t m = 0x6c078965 * (s ^ s >> 30) + 1;
		s = s & 0x80000000 | m & 0x7fffffff;
		for (int i = 2; i < 398; i++)
			m = 0x6c078965 * (m ^ m >> 30) + i;

		m ^= (s >> 1) ^ (s & 1 ? 0x9908b0df : 0x0);

		m ^= m >> 11;
		m ^= m << 07 & 0x9d2c5680;
		m ^= m << 15 & 0xefc60000;
		m ^= m >> 18;

		return (uint8_t)(!(m % 10));
	}

	uint8_t read_cache(uint32_t s) {
		return c[s >> 3] & (0x80 >> (s & 7));
	}

	void write_cache(uint32_t min, uint32_t max) {
		uint32_t n = min << 3;
		for (int i = min; i < max; i++) {
			uint8_t t = 0;
			for (int j = 7; j >= 0; j--) {
				t |= is_slime_chunk(n) << (j);
				n++;
			}
			c[i] = t;
		}
	}

	uint32_t get_seed(int32_t x, int32_t z) {
		return x * 0x1f1f1f1f ^ z;
	}

#ifdef __cplusplus
}
#endif

#endif
