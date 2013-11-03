.DEFAULT_GOAL := rcsv

libregex_native.a: regex_native.c
	gcc -O2 -c regex_native.c -o regex_native.o
	ar rcs libregex_native.a regex_native.o

rcsv: rcsv.rs regex.rs libregex_native.a
	rustc -O rcsv.rs

clean: 
	rm -v *.a
	rm -v *.o
	rm rcsv