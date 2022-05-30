#include <stdio.h>

volatile int example_global = 1;

int
main(void) {
    printf("Hello world!\n");
    example_global += 1;
    return 0;
}

