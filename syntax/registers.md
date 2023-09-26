mappings for the x86 registers, each architectures registers should translate to r1, r2, r3, ...   

DEVNOTE: this list is missing some special registers like the stack base ptr
```
rax - r1  
rbx - r2  
rcx - r3  
rdx - r4  
rsi - r5  
rdi - r6  
rsp - r7  
rbp - r8 
```

the max num of registers supported is 256

create a var called "foo" and have it occupy the r1 register
```
;r1 foo = 20
```

the register name may be ommited, in which case the compiler will just choose a one thats availble.  
This is not recommended as if all registers are full the compiler will throw an error, and it may lead to undefined behaviour  
  
this is the default max sized register, so for x86_64 r1 would have the size 8    
for accessing the differently sized subdivisons use one of the sub-registers (r1 is by default the same as r1q):     
```
;r1q foo = 20    // 8 byte - quad word
;r1d foo = 20    // 4 byte - double word
;r1w foo = 20    // 2 byte - word
;r1l foo = 20    // low 1 byte
;r1h foo = 20    // high 1 byte
```

trust you are fully aware of the implications of using bare registers in your code.  
when able use the stack and heap and let the compiler handle the register allocation.

