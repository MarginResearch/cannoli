all:
	mipsel-pc-linux-gnu-gcc -static -O2 -g -nostdlib test.c -o benchmark
	riscv64-unknown-linux-musl-gcc -static -O2 -g -nostdlib test.c -o benchmarkrv64

