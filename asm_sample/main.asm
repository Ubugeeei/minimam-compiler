global _main

_main:
    xor eax, eax
    xor ebx, ebx
    mov eax, 2
    mov ebx, 7
    add eax, ebx
    ret

; .sub:
;     xor eax, eax
;     xor ebx, ebx
;     mov eax, 10
;     mov ebx, 7
;     sub eax, ebx