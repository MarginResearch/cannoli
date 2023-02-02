# note: need apt -y install gcc-mipsel-linux-gnu
CFLAGS = -O0 -g -static -fno-pic

all: clean
	mipsel-linux-gnu-gcc -o ./backtrace_test_mips $(CFLAGS) ./backtrace_test.c
	nm ./backtrace_test_mips > ./symbols.txt

run_client:
	cargo +nightly run --release --bin backtrace

clean:
	rm -f ./backtrace_test_mips
	rm -f ./symbols.txt
