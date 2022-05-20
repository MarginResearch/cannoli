void
__start(void) {
    for( ; ; ) {
        asm volatile(".rept 100 ; nop ; .endr");
    }
}

void
_start(void) {
    __start();
}

