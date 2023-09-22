# Comments
c style
```
// single line comment

/*
multi
line
comment
*/
```

# Math and Logic
all math and logic expressions must be wrapped in `()`
```
(1 + 2)   // 3
```

as there's no dedicated bool type, a `false` is a `0` and a `true` is anything above  
those are macro'd to `0` and `1`

to `not` the expression prefix it with a `~` 
```
~(1 = 1)  // 0
```

# Conditionals
the `=>` operator will execute the right hand side if the left hand side is true
```
(1 = 1) => !$puts "math works"
```

the right hand side may also be a code block
```
(1 = 1) => {
    !$puts "math works"
    // do something
}
```

for an `else` type behaviour use the `|` operator
```
(1 = 2) => !$puts "math works" | !$puts "math doesn't work"
```

also works with blocks!


# Directives
directives are prefixed with a `.` and are used to control the compiler/assembler

## ent - entry point
the entry point of the program, the compiler will start execution at the label specified
```
.ent start

@start // execution starts here
```
by default the entry point is `@main`

## inc - include
include a file or a library.

```
.inc "some_file.shd"   // some_file.shd from the current directory
.inc "/some_file.shd"  // some_file.shd from the root directory
.inc std.io          // the io module from the library `std` from the system library directory
```

## txt - text
inserts the text into the output file
```
.txt "hello world"
```

## arch - architecture 
DEVNOTE: I'm unsure about this one  
sets the target architecture for this specific file.    
when compiling for any other architecture the file will be ignored.  

planned:  
```
.arch x86_64_linux
.arch x86_64_osx
.arch avr_atmega328p
.arch avr_atmega16
.arch avr_attiny45
```

## def - define
defines a variable-like macro to a literal
```
.def THIRTEEN 13

// later

(1 + THIRTEEN) // 14
```

## dat - static
creates a static variable  
```
.dat foo 8 = 20
```

it doesnt need a value, in which case it will be initialized to null  
```
.dat foo 8
```

# Macros
prefixed with a `&`. Basically a bit of code that expands at compiletime.

as an example this line:
```
&prt "hello, world!\n"
```
expands to this:
```
*stdout 14, "hello, world!\n"
```

keep in mind the expansion and calculations happen at compiletime, so the `&prt` macro will only work on literals.


# Markers
Markers, Labels, Subroutines, Functions, call em whatever.  
They are prefixed with a `@`, and operate in the same fasion as assembly label, but while allowing return and call arguments.
```
@main   // marker
    ;result = !add 3 4  
    // result = 7

@add x 1, y 1 -> 1   // function
    ret (x + y)
```

## Calling
use the `!` operator to call one, this means that it will be executed until the first `ret` (return) is hit.  
The execution then continues whenever the call was made.
```
@main
    !foo
    // continue this

@foo
    // do something
    ret
```

to call a global function prefix it with a `$`
same goes for labels and stuff
```
!$puts "hello world"
```

To pass a null arg:
```
!foo _
```

## Jumps
the `#` operator jumps to a label. This does not push a return adress to the stack so it's a one way trip.
```
@main
    #foo

@foo
    // do something
    #main // jump back to main
```

# Pointers and arrays
use `[]` to dereference a ptr

creating an array gives the pointer to the first element, cannot store them in registers  

to create one, either:
```
%foo 1:11 = "hello world"  // 11 element array from ascii, 1 byte each

%bar 2:4 = `1, 2, 4, 5`    // 4 element array from arbitrary data, 2 byte each
```

```
[array.0]   // first element
[array.1]   // second element
[array.2]   // third element
```

might make a macro for this later.. maybe

# Variables
The concept of variables needs no further introduction.  
Immutable vars are not a thing cause thats not how computers work :v  

## Mutating
the `'` operator is used to mutate vars. It expects the var name after it then the operation, and optionally second var
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
'foo ~ bar   // not bar and store in foo
'foo ++
'foo --
```

in most of these operations the second value is optional, in which case it will use the first value as the second as well.  
```
'foo + 20    // add 20 to foo
'foo +       // add foo to foo
```


## Registers
mappings for the x86 registers, each architectures registers should translate to r1, r2, r3, ...   
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

create a var called "foo" and have it occupy the r1 register
```
;r1 foo  = 20
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

to specify the registers used for a function manually  
```
@add x r1, y r2 -> 8
    ret 20
```

I trust you are fully aware of the implications of using bare registers in your code.  
when able use the stack and heap and let the compiler handle the register allocation.
cause the registers are modified by basically every func you call

## Stack
variables here must have a known size at compiletime.  

reserve 4 bytes on the stack and move `20` in there
```
%foo 4 = 20
```

unlike registers, `foo` isn't the actual value but an offset from the stack base pointer.
you need to dereference it to get the value
```
!$puts [foo]   // 20
```

to just reserve the space without assigning a var:
```
%foo 4       // for 1, 2, 4, 8 byte vars
%foo :55     // for a non standard size, like for stack arrays
'rs + 50     // move the stack pointer by 50 bytes
```

stack operations:
```
^20      // push 20
'r1 _    // pop into r1
'r1 ?    // peek into r1
```

ok the first line was a lie, you *can* allocate unknown size vars to the stack. Not saying you *should* but you *can*.   
Let me make this *very* clear, you will need to manually free the memory afterwards, the compiler won't do it for you.   
if you dont.. well you dont want to find out  

## Heap
there's no allocator implementation for this lang yet... *maybe* later.  
for now you can just call whatever allocator you like.  

here's an example using the libc malloc and free
```
;foo = !$malloc 20
// do something with foo
!$free foo
```

for further detail read the libc docs
