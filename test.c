#include <stdio.h>

#include "slimechunkfinder.h"

int main(void)
{
	slime_initialization();

	int x1, z1, x2, z2, n, thr;
	while (1)
	{
		char mode = getchar();
		switch (mode)
		{
		case 'h':
			printf("Slime chunk map: \tm x1 z1 x2 z2\n");
			printf("Slime chunk finder: \tf x1 z1 x2 z2 n thr\n");
			printf("Quit: \t\t\tq\n");
			printf("Help: \t\t\th\n");
			break;
		case 'm':
			scanf("%d %d %d %d", &x1, &z1, &x2, &z2);
			slime_map(x1, z1, x2, z2);
			break;
		case 'f':
			scanf("%d %d %d %d %d %d", &x1, &z1, &x2, &z2, &n, &thr);
			slime_finder(x1, z1, x2, z2, n, thr);
			break;
		case 'q':
			return 0;
		default:
			break;
		}
		putchar('\n');
	}
}
