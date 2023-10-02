these are for x86\_64, but the compiler will be able to target other architectures in the future, hopefuly
```
       |  8b  |  4b  |  2b  | 1 high | 1 low |
Instr  | r0q  | r0d  | r0w  |  ---   |  ---  |
Acc    | r1q  | r1d  | r1w  |  r1h   |  r1l  |
Base   | r2q  | r2d  | r2w  |  r2h   |  r2l  |
Count  | r3q  | r3d  | r3w  |  r3h   |  r3l  |
Data   | r4q  | r4d  | r4w  |  r4h   |  r4l  |
Source | r5q  | r5d  | r5w  |  ---   |  r5l  |
Dest   | r6q  | r6d  | r6w  |  ---   |  r6l  |
Stack  | r7q  | r7d  | r7w  |  ---   |  r7l  |  // stack pointer
SBptr  | r8q  | r8d  | r8w  |  ---   |  r8l  |  // stack base pointer
R8     | r9q  | r9d  | r9w  |  ---   |  r9l  |
R9     | r10q | r10d | r10w |  ---   |  r10l |
R10    | r11q | r11d | r11w |  ---   |  r11l |
R11    | r12q | r12d | r12w |  ---   |  r12l |
R12    | r13q | r13d | r13w |  ---   |  r13l |
R13    | r14q | r14d | r14w |  ---   |  r14l |
R14    | r15q | r15d | r15w |  ---   |  r15l |
R15    | r16q | r16d | r16w |  ---   |  r16l |
CodeS  | r17q | r17d | r17w |  ---   |  r17l |  // code segment
DataS  | r18q | r18d | r18w |  ---   |  r18l |  // data segment
ExtraS | r19q | r19d | r19w |  ---   |  r19l |  // extra segment
StackS | r20q | r20d | r20w |  ---   |  r20l |  // stack segment
FS     | ---- | ---- | r21w |  ---   |  ---- |  // general purpose F segment
GS     | ---- | ---- | r22w |  ---   |  ---- |  // general purpose G segment
EFLAGS | r23q | ---- | ---- |  ---   |  ---- |  // EFLAGS register
CR0    | r24q | ---- | ---- |  ---   |  ---- |  // control register 0
CR2    | r25q | ---- | ---- |  ---   |  ---- |  // page fault linear address
CR4    | r26q | ---- | ---- |  ---   |  ---- |  // control register 4
```

the max num of registers supported is 256

the size may be skipped, in which case the default will be the largest size for that register


create a var called "foo" and have it occupy the r1 register
```
;r1 foo = 20
```

the register name may be ommited, in which case the compiler will just choose a one thats availble. ( for syscall returns this will always be r1 )
This is not recommended as if all registers are full the compiler will throw an error, and it may lead to undefined behaviour  

trust you are fully aware of the implications of using bare registers in your code.  
when able use the stack and heap and let the compiler handle the register allocation.
