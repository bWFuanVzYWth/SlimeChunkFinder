#include <stdint.h>
#include <stdio.h>

#include "slimechunkfinder.h"

uint8_t c[L] = {0};

inline void swap(int *x1, int *z1, int *x2, int *z2)
{
	int t;
	if (*x1 > *x2)
	{
		t = *x1;
		*x1 = *x2;
		*x2 = t;
	}
	if (*z1 > *z2)
	{
		t = *z1;
		*z1 = *z2;
		*z2 = t;
	}
}

void slime_map(int x1, int z1, int x2, int z2)
{
	swap(&x1, &z1, &x2, &z2);

	for (int z = z1; z < z2; z++)
	{
		for (int x = x1; x < x2; x++)
		{
			uint8_t b = read_cache(get_seed(x, z));
			putchar(b ? '#' : ' ');
			putchar(' ');
		}
		putchar('\n');
	}
}

uint8_t f2[MAX_N][2 * MAX_W + MAX_N] = {0};
uint8_t f1[2 * MAX_W + MAX_N] = {0};

void slime_finder(int x1, int z1, int x2, int z2, int N, int thr)
{

	swap(&x1, &z1, &x2, &z2); //给输入的坐标范围排序

	const int X = x2 - x1;
	const int Z = z2 - z1;

	if (X > 2 * MAX_W || Z > 2 * MAX_W)
	{
		printf("Too large to find.");
		return;
	}

	for (int x = 0; x < X; x++)
	{
		int m = x % N;
		int num = 0;
		int xpos = x + x1;
		uint32_t tmp_get_seed = xpos * 0x1f1f1f1f;
		for (int z = 0; z < N; z++)
		{
			num += f1[z];
		}
		for (int z = N; z < Z; z++)
		{
			int zpos = z + z1;
			uint32_t seed = tmp_get_seed ^ zpos; //get_seed已被手动内联，优化性能

			f1[z] -= f2[m][z];
			f2[m][z] = read_cache(seed);
			f1[z] += f2[m][z];

			num -= f1[z - N];
			num += f1[z];

			if (num > thr)
				printf("x=%d,z=%d,n=%d\n", xpos * 16, zpos * 16, num);
		}
	}
}

inline uint32_t get_seed(int32_t x, int32_t z)
{
	return x * 0x1f1f1f1f ^ z;
}

inline uint8_t is_slime_chunk(uint32_t s)
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

void write_cache(uint32_t min, uint32_t max)
{
	uint32_t n = min << 3;
	for (int i = min; i < max; i++)
	{
		uint8_t t = 0;
		for (int j = 7; j >= 0; j--)
		{
			t |= is_slime_chunk(n) << (j);
			n++;
		}
		c[i] = t;
	}
}

uint8_t read_cache(uint32_t s)
{
	return (c[s >> 3] & (0x80 >> (s & 7))) != 0;
}

int slime_initialization(void)
{
	FILE *fp = NULL;

	fp = fopen(FC, "rb");
	if (fp == NULL)
	{
		printf("%s no found. Trying to regenerate.\n", FC);
		printf("Please wait about half an hour.\n");
		write_cache(0, L);
		fp = fopen(FC, "wb");
		if (fp == NULL)
		{
			printf("Can not write %s\n", FC);
			goto error;
		}
		fwrite(c, 1, L, fp);
		fclose(fp);
		printf("%s has been regenerated.\n", FC);
	}

	fp = fopen(FC, "rb");
	if (fp == NULL)
	{
		printf("Can not read %s\n", FC);
		goto error;
	}

	fread(c, 1, L, fp);
	fclose(fp);

	return 0;

error:
	printf("There seems to be something wrong in initialization.\n");
	return 1;
}