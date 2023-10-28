#include "slimechunk.h"

#define max(a, b) (((a) > (b)) ? (a) : (b))
#define min(a, b) (((a) < (b)) ? (a) : (b))

void slime_map(int x1, int z1, int x2, int z2, uint8_t* LUT) {
    const int XMIN = min(x1, x2);
    const int XMAX = max(x1, x2);
    const int ZMIN = min(z1, z2);
    const int ZMAX = max(z1, z2);
    const int DX = XMAX - XMIN;
    const int DZ = ZMAX - ZMIN;

    char* str = (char*)calloc(2 * (size_t)DX * (size_t)DZ, sizeof(char) + 1);
    char* write = str;

    for(int z = ZMIN; z <= ZMAX; ++z) {
        for(int x = XMIN; x <= XMAX; ++x) {
            *write++ = is_slime_chunk_LUT(to_seed(x, z), LUT) ? '#' : ' ';
            *write++ = x == XMAX ? '\n' : ' ';
        }
    }

    puts(str);
    free(str);
}

int main(void) {
    uint8_t* lut = generate_LUT();

    slime_map(-10, -10, 10, 10, lut);

    free(lut);

    return 0;
}