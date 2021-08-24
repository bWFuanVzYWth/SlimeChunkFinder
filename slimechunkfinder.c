
#include<stdint.h>
#include<stdio.h>

#define FC "slimechunk.bin" //file name of cache
#define L 0x20000000 //bytes used by the cache

#define FO "slimechunk.txt" //file name of output
#define W 0x800000 //max valid chunk position

	uint32_t get_seed(int32_t, int32_t);
	uint8_t is_slime_chunk(uint32_t);
	void write_cache(uint32_t, uint32_t);
	uint8_t read_cache(uint32_t);

	uint8_t c[L] = { 0 };

void test(void) {
	for (int z = -10; z < 10; z++) {
		for (int x = -10; x < 10; x++) {
			uint8_t b = read_cache(get_seed(x, z));
			putchar(b ? '#' : ' ');
			putchar(' ');
		}
		putchar('\n');
	}
}

int main(void) {
	FILE* fp = NULL;
	fp = fopen(FC, "rb");
	if (fp == NULL) {
		printf("%s no found. Trying to regenerate.\n", FC);
		printf("Please wait a few minutes.\n");
		write_cache(0, L);
		fp = fopen(FC, "wb");
		if (fp == NULL) {
			printf("Can not write %s\n", FC);
			goto error;
		}
		fwrite(c, 1, L, fp);
		fclose(fp);
		printf("%s has been regenerated.\n", FC);
	}
	fp = fopen(FC, "rb");
	if (fp == NULL) {
		printf("Can not read %s\n", FC);
		goto error;
	}

	fread(c, 1, L, fp);
	fclose(fp);

	test();

	return 0;

error:
	printf("There seems to be something wrong :(\n");
	return 1;

}

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
