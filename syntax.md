# Shard syntax

## Preface
Shard is syntactically similar to about nothing. 
As such, this document was created to provide a rough overview of the syntax in 
shard, assuming a basic understanding of C.
It will not cover coding techniques, best practices, or how to make pasta.
It will however cover the semantic meaning of each token as well as the grammar 
of shard whilst providing minimal examples.

I will also try to arrange this in order of importance, with links to previous 
sections when mentioned.

# Key concepts
## Sizes
Shard's mast widely used concept is __Sizes__, these refer to the size of some data in bytes.
//! idk say something more? is there even anything more to say?

## Registers
Registers in Shard map directly to the architecture's native registers,
Allowing to quickly store a value within a single clock cycle.

They can be adressed by using Shard's register notation which begins with an `r`,
then a number between 0 and 255, and optionally a size in the form of a letter.
Not including the latter means the architecture's native size will be used.

```
|   | Meaning     |Size|
|---|-------------|----|
| q | quad word   | 8  |
| d | double word | 4  |
| w | word        | 2  |
| l | low byte    | 1  |
| h | high byte   | 1  |
```


//! The architecture specific header shouldn't be mentioned here and prob should have it's own section.
//! Also the headers are optional, making them not the default behaviour.
This table would apply for adressing the registers for the `x86_64` architecture
```
| Name   |8 byte|4 byte|2 byte| 1 high | 1 low | Description               |
|--------|------|------|------|--------|-------|---------------------------|
| Instr  | r0q  | r0d  | r0w  |  ----  |  ---- |                           |
| Acc    | r1q  | r1d  | r1w  |  r1h   |  r1l  |                           |
| Base   | r2q  | r2d  | r2w  |  r2h   |  r2l  |                           |
| Count  | r3q  | r3d  | r3w  |  r3h   |  r3l  |                           |
| Data   | r4q  | r4d  | r4w  |  r4h   |  r4l  |                           |
| Source | r5q  | r5d  | r5w  |  ----  |  r5l  |                           |
| Dest   | r6q  | r6d  | r6w  |  ----  |  r6l  |                           |
| Stack  | r7q  | r7d  | r7w  |  ----  |  r7l  | stack pointer             |
| SBptr  | r8q  | r8d  | r8w  |  ----  |  r8l  | stack base pointer        |
| R8     | r9q  | r9d  | r9w  |  ----  |  r9l  |                           |
| R9     | r10q | r10d | r10w |  ----  |  r10l |                           |
| R10    | r11q | r11d | r11w |  ----  |  r11l |                           |
| R11    | r12q | r12d | r12w |  ----  |  r12l |                           |
| R12    | r13q | r13d | r13w |  ----  |  r13l |                           |
| R13    | r14q | r14d | r14w |  ----  |  r14l |                           |
| R14    | r15q | r15d | r15w |  ----  |  r15l |                           |
| R15    | r16q | r16d | r16w |  ----  |  r18l |                           |
| CodeS  | ---- | ---- | r17w |  ----  |  ---- | code segment              |
| DataS  | ---- | ---- | r18w |  ----  |  ---- | data segment              |
| ExtraS | ---- | ---- | r19w |  ----  |  ---- | extra segment             |
| StackS | ---- | ---- | r20w |  ----  |  ---- | stack segment             |
| FS     | ---- | ---- | r21w |  ----  |  ---- | general purpose F segment |
| GS     | ---- | ---- | r22w |  ----  |  ---- | general purpose G segment |
| EFLAGS | r23q | ---- | ---- |  ----  |  ---- | EFLAGS register           |
| CR0    | r23q | ---- | ---- |  ----  |  ---- | control register 0        |
| CR2    | r24q | ---- | ---- |  ----  |  ---- | page fault linear address |
| CR4    | r25q | ---- | ---- |  ----  |  ---- | control register 4        |
```

//! Should this be it's own section? prob check how a `###` renders
### Register Aliasing
Shard allows for something called "register aliasing", ie. temporarily assigning 
a specific name to the register for it to be easily differentiated from. 
Functionally an aliased register works the same as directly adressing the register.

While aliasing a register you may also move a value into it, making it work as kind
of a "register allocated varaible".

```
;r3w bar       // alias `r3w` as `bar`
;r1 foo = 20   // alias `r1` as `foo` and set it to `20`
```

In a case that instead of the register name a question mark `?` is given, the compiler
will try to choose an unaliased register from the architecture's "general registers".
//# the general registers thing is here so it doesnt auto choose a control register or something

I trust you are fully aware of the implications of using bare registers in your code.


## Types
In Shard, types are actually just sizes (number of bytes) with some extra 
information as to how data is stored inside these sizes.
There are three kinds of type: _Buffer_, _Segmented_ and _Pointer_. There is 
also the [_Structure_](#Structs) type, which will be covered in a later section.

_Buffer types_ contain any sort of data, in a fixed amount of space. For 
instance, if i wanted to make a buffer of length 11, the type would simply be 
`11`.

_Segmented types_ contain some sort of structured data, with segments being 
homogeneous in size, in a fixed amount of space.
An array with three elements, each one byte in length would be have a type of 
`3:1`.
This is because segmented types are defined as such.

```
 3:1
 | |- Each element's size in bytes
 |- Number of elements
```

_Pointer types_ are composite in the sense that they can contain a segmented or 
a buffer type.
They contain a pointer, of the same size as the machine's architecture, which 
points to the start of a buffer (which may or may not be segmented).
To create a pointer to a buffer of length 20, the type would be `[20]` and a 
pointer to an array with 5 elements, each 4 bytes in length would have a type 
of `[5:4]`.

Below are some example types, annotated with their meaning:
```
|Shard|Description                        |
|-----|-----------------------------------|
|1    |1 byte                             |
|2    |2 bytes                            |
|4    |4 bytes                            |
|8    |8 bytes                            |
|5:4  |5 elements, 4 bytes each           |
|:55  |55 bytes                           |
|[]   |void pointer                       |
|[4]  |pointer to 4 bytes                 |
|[4:8]|pointer to 4 elements, 8 bytes each|
```

## Literals
Literals in Shard are similar to other languages however there are some 
differences:

Strings in shard always contain a `\0`, even if they are empty. "" is 
equivalent to one byte  of memory, with a `\0` byte at the end. To circumvent 
this, you can use a raw string `r""` which will not include the null byte at 
the end, and will also not escape anything. Chars are enclosed by the 
back-tick (`` ` ``) character instead of the apostrophe(`'`) like in C. The 
only restriction is that it must be at most a byte. Both strings and 
characters can use c-style escape codes by using the backslash character. 

Arrays are defined by a list of comma seperated values inside of squiggley 
braces `{}`. The resulting value will be a pointer to the first item in the 
array, similar to how a string points to the first character in its array.

Intagers are defined exactly the same as how C defines them. Since there are
no types, distinction between negative and positive intagers is impossible.
Shard does allow you to denote negative intagers by pre-pending intagers with 
a minus sign (`-`) to store the number in two's complement. Any further use
of a signed intager will have to be checked by the programmer as only unsigned
operators are available in Shard. The standard x64 library does provide signed
arithmetic functions for this.

Below are some examples of literals, with c equivalents, and a description of 
what they are. These assume that the size of the buffer they are being stored
into es exactly 8 bytes long.

| Literal | C equivalent | Description  | Type (Size) |
|---------|--------------|--------------|-------------|
|"Hello"  |"Hello\0"     |char pointer  | [6:1]       |
|r"Hello" |"Hello"       |char pointer  | [5:1]       |
|\`c\`    |'c'           |char          | 1           |
|\`\t\`   |'\t'          |char (tab)    | 1           |
|\`\x10\` |'\x10'        |char (newline)| 1           |
|{1, 3, 4}|{1, 3, 4}     |array pointer | [3:1]       |
|20       |20            |int (20 dec)  | 1           |
|0b1010   |0b1010        |int (10 bin)  | 1           |
|0x0a     |0x0a          |int (10 hex)  | 1           |
|-4       |-4            |int (-4 dec)  | 1           |


# Comments
Shard has both, `// inline` and `/* block */` c-style comments.
They work just as comments in any other __sane__ language would and are ignored 
during compilation.


# Math Blocks
The Math Block (or Logic Block) in Shard is used to make the destinction between 
math and logic operators and shard specific ones, and to alter the order of 
operations within other math expressions.
//! mention how to make and use it, and how it may cause UB in register heavy contexts


# Conditionals
Conditionals are Shard's type of `if` statements from other languages.

```
expression_to_test => statement_to_execute
```

to negate a value or the entire experession, use `~`
```
~(1 = 1) => $puts "math doesn't work"
```

the `->` operator will execute the right hand side if the left hand side is true
```
(1 = 1) -> !$puts "math works"
```

here's how you'd do an `else`
```
(1 = 1) -> $puts "math works" | $puts "math doesn't work"
```

# Arrays
creating an array gives the pointer to the first element, cannot store them in 
registers  

to create one, either:
```
%bar 1:12 = "hello world"   // 12 byte array from string
%bar 4:2 = {1, 2, 4, 5}    // 4 element array from arbitrary data, 2 byte each
```

the first number is the num of elements, the second is the size of each element
the num of elements may be ommited to represent arbitrary continous data
```
// like this:
%bar 4:1 = 0*     // 4 element array from arbitrary data, 1 byte each
```
the `0*` shorthand is described in  the [general](#general) section.

```
[array.0]   // first element
[array.1]   // second element
[array.2]   // third element
```

# Directives
## def - define
defines a variable-like macro, this is a literal compile-time copy-paste
```
.def THIRTEEN 13

(1 + THIRTEEN) // 14
```
the last line evaluates to
```
(1 + 13) // 14
```

## ent - entry
the entry point of the program, the compiler will start execution at the label 
specified
```
.ent start

start: // execution starts here
```
by default the entry point is `main`


## inc - include
include a file or a library.

```
.inc "some_file.shd"   // some_file.shd from the current directory
.inc "/some_file.shd"  // some_file.shd from the root directory
.inc std.io          // the io module from the library `std` from the system 
library directory
.inc std.io.prtl     // the prtl function from the io module from the std 
library
```


## con - constant
a read only variable integrated into the final binary, gives a pointer
```
.con TEN 4 = 10

(1 + [TEN]) // 11
```


## dat - data
basically a static var
```
.dat FOO 8 = 20
```

it doesnt need a value, in which case it will be initialized to null  
```
.dat foo 8
```

# .str - struct 
its a struct
```
.str Foo {
    8 bar  // 8 bytes
    1 baz  // 1 byte 
    4 fiz  // 4 bytes
}

#main
    %some_var Foo = { 20, 1, 10 }
    '[some_var.bar] = 20
    '[some_var.baz] = 1
    '[some_var.fiz] = 10
```

to access diff fields use `.` like indexing arrays



# .txt - text
inserts text into the binary. This is done through creating a constant but 
ignoring the pointer to it.
```
.txt "Hello World!"
```

# .mac - macro
evaluated at compiletime
```
.mac add x, y {
    (x + y)
}

/add 1, 2   // (1 + 2)
```

# Functions
A function is an abstract self-contained block of code with a defined scope, 
name, beginning, and end.  
to preserve the sequential execution core pillar functions must be defined 
before any label in the program.

this also means that you can have a "function file" akin to C's header files 
but just for functions

to define one use:
```
@hello {
    $puts "Hello, World!"
}
```

they may also accept arguments.. well that's the reason ye'd use one in the 
first place :p  
the args must always have a size
```
@add x 2, y 2 -> 2 {
    <- x + y     // return x + y
}
```

keep in mind this is the ONLY exception to the rule of sequential execution, 
the compiler will not execute the function until it is called.  

to call one use:
```
*hello
// or
*add 1, 2
```

additionally functions may have attributes:
```
|inline, ignore|
@add x 2, y 2 -> 2 {
    <- x + y
}
```

### Function attributes

#### inline `|inline|`
the inline attribute will cause the compiler to replace the function call with 
the function body, this is useful for small functions that are called often.

#### macro `|macro|`
converts the function into a compiletime macro. use `/` to call it

#### ignore `|ignore|`
stops warnings about unused functions.

# General
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

other
```
true   // 1 
false  // 0
```

## Sizes
```
b = byte = 8 bits
w = word = 16 bits
d = dword = 32 bits
q = qword = 64 bits
```


## Default types for values
```
int = 1
char = 1
ptr = 8 // on x86_64, for other architectures it may be different
```

## Currently Used Tokens
All ASCII characters, including space, newline, and EOF.

### double char tokens
```
--   // decrement
++   // increment
<=   // lesser_equal
>=   // greater_equal
~=   // not_equal
->   // arrow_right
<-   // arrow_left
```

### keywords
```
ret  // return
jmp  // jump
end  // end
```

## general rules
all directives and functions must be defined before any label in the program.

# Heap
There's no allocator implementation, use whichever you want.
here's an example with `malloc()` and `free()`:
```
;ptr = $malloc 4

$printf "ptr: %h" ptr

free ptr
```

# Interrupts
### stdout
```
&stdout 14, "hello, world!\n"
```
1st arg is the len, second a ptr

### stderr
```
&stderr 14, "hello, world!\n"
```
1st arg is the len, second a ptr

# Labels
a label is a named location in code, you may call it or diretly jump to it.

```
#loop    // define label
jmp loop // jump
!loop    // call 
ret      // return
```

# Mutations
changes the 1st arg (foo), optionally using the 2nd arg (bar) as a value

```
'foo = 20    // set foo to 20
'foo + 20    // add 20 to foo
'foo - 20    // subtract 20 from foo
'foo ^ bar   // xor foo with bar
'foo & bar   // and foo with bar 
'foo | bar   // or foo with bar 
'foo : bar   // same as '[foo] = [bar]
'foo < 20    // shift foo left by 20
'foo > 20    // shift foo right by 20
'foo ! bar   // not bar and store in foo
'foo ++      // increment foo
'foo --      // decrement foo
'foo _       // pop stack into foo
'foo ?       // peek stack into foo
```

in most of these operations the second value is optional, in which case it will 
use the first value as the second as well.  
```
'foo + 20    // add 20 to foo
'foo +       // add foo to foo
```


# Stack 
variables here must have a known size at compiletime.  

reserve 4 bytes on the stack and move `20` in there
```
%foo 4 = 20
%foo ? = 20  // same thing, but the size is inferred from the value
```

unlike registers, `foo` isn't the actual value but an offset from the stack 
base pointer.
you need to dereference it to get the value
```
$puts [foo]   // 20
```

to just reserve the space without assigning a var:
```
%foo 4       // for 1, 2, 4, 8 byte vars
%foo 2:8     // for arrays, 2 elements, 8 bytes
'r$ + 50     // move the stack pointer by 50 bytes
```

stack operations:
```
^20 8    // push 20, size 8
'r1 _    // pop into r1
'r1 ?    // peek into r1
```

ok the first line was a lie, you *can* allocate unknown size vars to the stack. 
Not saying you *should* but you *can*.   
Let me make this *very* clear, you will need to manually free the memory 
afterwards, the compiler won't do it for you.   
if you dont.. well you dont want to find out

# Statics
prefixed with /, always global
```
/foo 8 = 20
```

it doesnt need a value, in which case it will be initialized to null  
```
/foo 8
```
