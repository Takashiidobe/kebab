bin/kebab: main.c
	cc -O2 main.c -o bin/kebab

clean:
	rm bin/kebab
