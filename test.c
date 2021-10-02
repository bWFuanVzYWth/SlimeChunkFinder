#include "slimechunkfinder.h"

int main(void)
{
    slime_initialization();
    while (1)
    {
        putchar('\n');
        int x1, z1, x2, z2, n, thr;
        switch (getchar())
        {
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
    }
}