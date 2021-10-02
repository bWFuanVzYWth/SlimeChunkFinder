#include "slimechunkfinder.h"

fast_t lut[LUT_LEN] = {0};

fast_t (*is_slime_chunk)(uint32_t s);

fast_t is_slime_chunk_lut(uint32_t s)
{
    return (lut[s / FAST_BITS] & ((1 << (FAST_BITS - 1)) >> (s % FAST_BITS))) != 0;
}

fast_t is_slime_chunk_mt(uint32_t s)
{
    uint32_t m = 0x6c078965 * (s ^ s >> 30) + 1;
    s = s & 0x80000000 | m & 0x7fffffff;
    for (int i = 2; i < 398; i++)
        m = 0x6c078965 * (m ^ m >> 30) + i;
    m ^= (s >> 1) ^ ((-((int32_t)(s & 1))) & 0x9908b0df);
    m ^= m >> 11;
    m ^= m << 07 & 0x9d2c5680;
    m ^= m << 15 & 0xefc60000;
    m ^= m >> 18;
    return (fast_t)(!(m % 10));
}

void generate_lut(void)
{
#pragma omp parallel for schedule(static)
    for (uint32_t i = 0; i < LUT_LEN; i++)
    {
        uint32_t seed = i * FAST_BITS;
        fast_t tmp = 0;
        for (uint32_t j = 0; j < FAST_BITS; j++)
        {
            tmp <<= 1;
            tmp |= is_slime_chunk_mt(seed | j);
        }
        lut[i] = tmp;
    }
}

uint32_t get_seed(int32_t x, int32_t z)
{
    return x * 0x1f1f1f1f ^ z;
}

void swap(int *a, int *b)
{
    if (*a > *b)
    {
        int tmp = *a;
        *a = *b;
        *b = tmp;
    }
}

int slime_initialization(void)
{
    FILE *fp;
    fp = fopen(LUT_NAME, "rb");
    if (fp != NULL)
    {
        fread(lut, FAST_SIZE, LUT_LEN, fp);
        fclose(fp);
        is_slime_chunk = is_slime_chunk_lut;
        printf("%s has been loaded.\n", LUT_NAME);
        return 0;
    }
    printf("%s no found. Regenerate? y/n\n", LUT_NAME);
    if (getchar() != 'y')
    {
        is_slime_chunk = is_slime_chunk_mt;
        printf("Running without %s.\n", LUT_NAME);
        return 0;
    }
    generate_lut();
    fp = fopen(LUT_NAME, "wb");
    if (fp == NULL)
    {
        printf("ERROR: Can not write %s\n", LUT_NAME);
        return 1;
    }
    fwrite(lut, FAST_SIZE, LUT_LEN, fp);
    fclose(fp);
    is_slime_chunk = is_slime_chunk_lut;
    printf("%s has been regenerate.\n", LUT_NAME);
    return 0;
}

void slime_map(int x1, int z1, int x2, int z2)
{
    swap(&x1, &x2);
    swap(&z1, &z2);

    int dx = x2 - x1 + 1;
    int dz = z2 - z1 + 1;

    char *str = (char *)calloc(2 * (size_t)dx * (size_t)dz, sizeof(char));
    char *write = str;

    for (int z = z1; z <= z2; ++z)
    {
        for (int x = x1; x <= x2; ++x)
        {
            *write++ = is_slime_chunk(get_seed(x, z)) ? '#' : ' ';
            *write++ = ' ';
        }
        *(write - 1) = '\n';
    }
    *(write - 1) = '\0';

    puts(str);
    free(str);
}

void slime_finder(int x1, int z1, int x2, int z2, int n, int thr)
{
    time_t t0, t1;

    swap(&x1, &x2);
    swap(&z1, &z2);

    int dx = x2 - x1 + 1;
    int dz = z2 - z1 + 1;

    t0 = clock();

#pragma omp parallel
    {
        int thread_num = omp_get_num_threads();
        int thread_id = omp_get_thread_num();

        int x_w = dx / thread_num + ((dx % thread_num) != 0);
        int x_1 = x1 + thread_id * x_w;
        int x_2 = x_1 + x_w - 1;
        x_2 = x_2 > x2 ? x2 : x_2;

        printf("thread %d : from %d to %d.\n", thread_id, x_1, x_2);

        x2 += n;

        uint8_t *f1 = (uint8_t *)calloc(dz, 1);
        uint8_t *f2 = (uint8_t *)calloc(n * dz, 1);

        for (int x = 0; x < (x_2 - x_1); x++)
        {
            int m = x % n;
            int num = 0;
            int xpos = x + x_1;
            uint32_t tmp_get_seed = xpos * 0x1f1f1f1f;
            int tmp_a = m * dz;
            for (int z = 0; z < n; z++)
            {
                num += f1[z];
            }
            for (int z = n; z < dz; z++)
            {
                int zpos = z + z1;
                uint32_t seed = tmp_get_seed ^ zpos;

                int a = tmp_a + z;
                f1[z] -= f2[a];
                f2[a] = is_slime_chunk(seed);
                f1[z] += f2[a];

                num -= f1[z - n];
                num += f1[z];

                if (num > thr)
                    printf("x=%d,z=%d,n=%d\n", xpos * 16, zpos * 16, num);
            }
        }
        free(f1);
        free(f2);
    }

    t1 = clock();

    double time = (double)(t1 - t0) / 1000.0;
    uint64_t speed = ((uint64_t)dx * (uint64_t)dz) / time;
    printf("finish in %.3lfs, %llu chunks/s\n", time, speed);
}
