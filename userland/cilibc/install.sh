sys=src/sys/x86_64

rm -rf ../sysroot/lib
mkdir -pv ../sysroot/lib
cp -v build/libc.a ${sys}/crt0.o ${sys}/crti.o ${sys}/crtn.o ../sysroot/lib/
