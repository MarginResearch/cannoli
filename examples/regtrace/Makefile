CFLAGS = -O0 -g -static -fno-pic

all: clean
	mipsel-unknown-linux-musl-gcc -o example_app $(CFLAGS) example_app.c

run32:
	QEMU_CANNOLI=../../target/release/libregtrace.so ~/qemu/build/qemu-mipsel ./example_app

clean:
	rm -f example_app

