## Usage

### compile to assembles and run

```rust
// main.rs
fn main() {
    // input your expression
    // ex)
    // " 12 + 3 ; "
    // "15 - 6 ;"
    // "5 * 9 ; "
    // " 24 / 12 ;"
    let tokens = tokenize(" 12 / 3 ; ");
    let expr = parse(&tokens, 0);
    generate_expr(&expr);
}
```

```sh
$ brew install nasm
```

```sh
$ cargo run | bash assemble.sh
$ ./a.out
$ echo $?
```

### build assembly sample

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
