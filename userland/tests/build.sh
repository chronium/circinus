rm test
x86_64-unknown-circinus-gcc -static -no-pie -fno-exceptions test.c -o test

rm -v ../build/test
cp -v test ../build
