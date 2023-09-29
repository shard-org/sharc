```
.   // directive

#   // define label
jmp // jump label
!   // call label
ret // return label

@   // define function
<-  // return function
*   // call function
|   // function attribute
/   // call macro function
end // return from a function

$   // call external

'   // mutations
;   // register assignment
%   // stack assignment

&   // syscall

()  // math and logic expression
[]  // dereference, may do math and logic within
{}  // code block

=>  // conditional

~   // negation
```

# literals
```
"hello"   // string ptr
`c`       // char
{1, 3, 4} // array ptr
20        // int
b1010     // binary
0x0a      // hex
```

other
```
true   // 1 
false  // 0
```

# Sizes
```
b = byte = 8 bits
w = word = 16 bits
d = dword = 32 bits
q = qword = 64 bits
```

# Types
```
1     // 1 byte
2     // 2 bytes
4     // 4 bytes
8     // 8 bytes
5:4   // 5 elements, 4 bytes each
:55   // 55 bytes
[]    // null pointer
[4]   // pointer to 4 bytes
[4:8] // pointer to 4 elements, 8 bytes each
```


## Default types for values
```
int = 1
char = 1
ptr = 8 // on x86_64, for other architectures it may be different
```

# Currently Used Tokens
All ASCII characters, including space, newline, and EOF.

### double char tokens
```
=>   // far_arrow_right
--   // decrement
++   // increment
<=   // fat_arrow_left
>=   // greater_equal
!=   // not_equal
->   // arrow_right
<-   // arrow_left
```

### keywords
```
ret  // return
jmp  // jump
end  // end
```

# general rules
all directives and functions must be defined before any label in the program.

