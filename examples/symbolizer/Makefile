CFLAGS = -O0 -g -static -fno-pic

all: clean
	mipsel-pc-linux-gnu-gcc -o example_app $(CFLAGS) example_app.c
	riscv64-unknown-linux-musl-gcc -o example_app64 $(CFLAGS) example_app.c
	nm example_app64 > symbols.txt
	nm example_app >> symbols.txt

run32:
	~/qemu/build/qemu-mipsel -cannoli /home/pleb/cannoli/target/release/libcannoli_server.so ./example_app

run64:
	~/qemu/build/qemu-riscv64 -cannoli /home/pleb/cannoli/target/release/libcannoli_server.so ./example_app64

clean:
	rm -f example_app example_app64

