pub static ASM_HEADER: &str = 
"section .text
    global _start\n";

// needs the text in `rax`, the type of print in `rdx`
// null terminated
pub static ASM_PRINT_STDOUT: &str = 
"_print:
    push rax
    push rdx
    mov rbx, 0
_print_loop:
    inc rax
    inc rbx
    mov cl, [rax]
    cmp cl, 0
    jne _print_loop

    mov rax, 1
    pop rdi
    pop rsi
    mov rdx, rbx
    syscall

    ret\n";

// expected to be jumped to, exit code in `rdi`
pub static ASM_EXIT: &str = 
"_exit:
    mov rax, 60
    syscall\n"

