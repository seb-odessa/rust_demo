all:lib obj bin run

lib:
	rustc rust_add.rs

obj:
	cc -c usage.c -o usage.o

bin:
	cc -o usage usage.o librust_add.so

run:
	LD_LIBRARY_PATH=. ./usage

clean:
	rm *.so
	rm *.o
	rm usage