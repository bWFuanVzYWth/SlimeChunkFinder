#include<stdint.h>
#include<stdio.h>

#include"slimechunkfinder.h"

#define FO "slimechunk.txt" //file name of output
#define W 0x800000 //max valid chunk position

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