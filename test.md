```rs
:ARCHDEF x86 linux

:WORD 8

:ATTR "static" "@ .data"
:ATTR "const" "@ .rodata"
:ATTR "init" "@ .bss"

:REG r0d "rax"
:REG r0l "eax"
/* ... */

:REG r5d "rdi"
:REG r4d "rsi"
/* ... */

:SYSCALL_ADDR 0x80
:SYSCALL_CONV r0, r5, r4, r3, r10, r9, r8 -> r0

:SYSCALL read { 
    = 0x00
    4 "File Descriptor"
    [1] "Buffer"
    #WORD "Buffer Size"
}

:SYSCALL write { 
    = 0x01
    4 "File Descriptor"
    [1] "Buffer"
    #WORD "Buffer Size"
}

// prtnt - print null terminated
prtnt string [1] {
    %len 8 = 0
    loop (string.len ~= 0):
        'len ++

    *write 1, string, len
}

prt inline string [1], len 4 {
    *write 1, string, len
}
```



static {
    VAR 1: 87
}

main entry:
   %two 4 = 8   // -> 8bit (byte)

   ;ident r8l = 80



   loop (two = 2) {
       $puts "hello!"
   }



// WARN: No way to check if the label always return at comptime if it doesnt have a body
add a 4, b 4 -> 4: 
   end a + b
   




int add(int a, int b) {
   return a + b;
}

```


###########
Notes

signed literals? nah


shadow identifiers: solves loops problem!
just allow returns with no body but comptime WARN
typed labels! `VAR 1: 87`
