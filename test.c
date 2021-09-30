#include <stdint.h>
#include <stdio.h>

#include "slimechunkfinder.h"

int main(void) {
    slime_initialization();

	//slime_map(-10, -10, 10, 10);
	slime_finder(-625, -625, 625, 625, 6, 13);
	//slime_finder(-524288, -524288, 524288, 524288, 12, 40);

	return 0;
}
