int
main(void) {
    for(int ii = 0; ii < 10000000; ii++) {
        asm volatile(".rept 100 ; nop ; .endr");
    }

    return 0;
}

