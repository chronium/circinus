set -e

target=x86_64-unknown-circinus

sys=src/sys
sys64=${sys}/x86_64

${target}-as ${sys64}/crt0.S -o ${sys64}/crt0.o
${target}-as ${sys64}/crti.S -o ${sys64}/crti.o
${target}-as ${sys64}/crtn.S -o ${sys64}/crtn.o
${target}-as ${sys64}/syscall.S -o ${sys64}/syscall.o

${target}-gcc -Iinclude/ -c ${sys}/exit.c -o ${sys}/exit.o

mkdir -pv build

x86_64-unknown-circinus-ar rcs build/libc.a ${sys64}/crt0.o ${sys64}/crti.o ${sys64}/crtn.o ${sys64}/syscall.o ${sys}/exit.o
