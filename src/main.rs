fn main() {
    println!("global _main");
    println!("\n");
    println!("_main:");
    println!("    xor eax, eax");
    println!("    xor ebx, ebx");
    println!("    mov eax, 2");
    println!("    mov ebx, 7");
    println!("    add eax, ebx");
    println!("    ret");
}
