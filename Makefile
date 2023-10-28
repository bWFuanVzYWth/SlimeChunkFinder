CC := gcc

CFLAGS := -static -fexec-charset=GBK -Wall -Wextra -Ofast -flto -pipe -march=native -mtune=native -fopt-info -fopenmp

test: slimechunk.c slimechunk.h main.c
	$(CC) -o $@ $^ $(CFLAGS)

clean:
	rm *.exe