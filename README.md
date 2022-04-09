## Usage

build assembly sample

```sh
$ brew install nasm
```

```sh
$ cd asm_sample
$ nasm -f macho64 -o main.o main.asm
$ ld -arch x86_64 -macosx_version_min 10.11 main.o -lSystem
```

run

```sh
$ ./a.out
$ echo $?
```
