gcc -c -Wall -Werror -fpic clet.c
gcc -shared -o libclet.dll clet.o