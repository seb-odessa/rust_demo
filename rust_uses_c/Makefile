obj:clang_add.c
	cc -c clang_add.c -o clang_add.o

lib:clang_add.o
	cc -shared clang_add.o -o libclang_add.so

cbin:
	cc -o usage usage.c libclang_add.so

crun:
	LD_LIBRARY_PATH=. ./usage

clean:
	rm *.so
	rm *.o
	rm usage