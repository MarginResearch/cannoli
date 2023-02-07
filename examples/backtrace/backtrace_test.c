#include "stdlib.h"
#include "stdint.h"
#include "stdio.h"

void function_two() {
    printf("Hello from function_two\n");
}

void function_one() {
    puts("Hello from function_one\n");
    function_two();
}

int main() {
    puts("Running test case program\n");
    puts("Calling fuction_two\n");
    function_two();
    puts("Now calling function_two via function_one\n");
    function_one();
    return 0;
}
