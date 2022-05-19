void
__start(void) {
    for(int ii = 0; ii < 10000000; ii++) {
        asm volatile(".rept 100 ; nop ; .endr");
    }

    // Crash to exit!
    *(volatile char*)1 = 1;
}

