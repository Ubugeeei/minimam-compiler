#!/bin/bash

cat > a.asm
echo '--- a.s ---'
cat a.asm

nasm -f macho64 -o a.o a.asm
ld -arch x86_64 -macosx_version_min 10.11 a.o -lSystem
