#include "slimechunkfinder.h"

#define TRY_SCANF(n, func)       \
    if (func != n)               \
    {                            \
        puts(WRONG_COMMAND_DOC); \
        break;                   \
    }

int main(void)
{
    int error = 0;

    if (error = slime_initialization())
        goto catch_error;

    puts(WELCOME_DOC);

    while (1)
    {
        putchar('\n');

        int x1, z1, x2, z2, n, thr;
        switch (getchar())
        {
        case 'm':
            TRY_SCANF(4, scanf("%d %d %d %d", &x1, &z1, &x2, &z2));
            slime_map(x1, z1, x2, z2);
            break;
        case 'f':
            TRY_SCANF(6, scanf("%d %d %d %d %d %d", &x1, &z1, &x2, &z2, &n, &thr));
            slime_finder(x1, z1, x2, z2, n, thr);
            break;
        case 'h':
            puts(HELP_DOC);
            break;
        case 'q':
            return 0;
        default:
            break;
        }
    }

catch_error: //直接退出吧，我摆烂
    return error;
}