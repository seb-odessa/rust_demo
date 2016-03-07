all:lib obj bin run

lib:rust_add.rs
	rustc rust_add.rs

obj:usage.o
	cc -c usage.c -o usage.o

bin:
	cc -o usage usage.o librust_add.so

run:
	LD_LIBRARY_PATH=. ./usage

clean:
	rm *.so
	rm *.o
	rm usage