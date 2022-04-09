#!/bin/fish

cat > a.asm
echo '--- a.s ---'
cat a.asm
echo '--- result ---'
nasm -f macho64 -o a.o a.asm
ld -arch x86_64 -macosx_version_min 10.11 a.o -lSystem
./a.out
echo $status
