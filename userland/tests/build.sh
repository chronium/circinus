rm test
x86_64-unknown-circinus-gcc -static -no-pie -fno-exceptions test.c -o test

cp -v test ../build
