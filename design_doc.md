#=============================
#||        Priority         ||
#=============================

- Loops
- definitions
- comments
- integers



#=============================
#||         Basics          ||
#=============================

# Variables and Statics =======

Create a variable like this:
;name = value

You may also specify a type, although this is optional
;name::type = value

for creating a static variable use:
;!name = value

a type may be specified as well
;!name::type = value

for integers and floats a bit length must be specified (default to 8/16 bit)
;name::<length> = value

by default all variables are global and have to be freed manually, 
or whenever the main starting subroutine goes out of scope
#name
or free all with:
#ALL
free all within this scope with:
#

to make a var bound to its scope, and automatically freed at the end of it:
;#name = value

# Subroutines ================
code execution begins at

main {

}

call one by just typing it's name:
main

heres an example of a subroutine that accepts two 8bit inputs and returns a single 16 bit one
add<2:8(x, y)> -> <16> {
    <- (x + y)

    // ret may also be used to return from a subroutine
    ret<(x + y)>
}

you would call it with
main {
    ;sum = add<1, 8>
    
    // to then print the sum
    // fmt formats things as ascii
    *stdout <- fmt<sum>

    // you may also print it directly
    // we also format it to utf8
    *stdout <- fmt::utf8<> <- add<1,8>
}

# Logic/Math operations
always surrounded by ()

for logic 
1 = true
0 = false



# Basic instructions ===========






#=============================
#||   Types and Notation    ||
#=============================
most types are just represented as collections bits
while there are higher level abstractions like Structs, Enums, Arrays, or Closures
all operations work on the bit level, so adding two characters together would just add their specific bytes
just like adding two 8bit integers would

# Primitives ===================

# Integers
Hex - 0x43
;foo = 0x43

Dec - 87
;foo = 87

Bin - 01010111b
;fooo = 01010111b

bit lengths:
- 8
- 16
- 32
- 64

# Chars
ascii - 8bits per char (7bits char + 1bit parity) DEFAULT // may disable parity bit?
;foo = 'b'    
    
utf-8 - 32bits per char (4x8bits char)
;foo::&::utf8 = '한'

# floats
bit lengths:
- 16  - half
- 32  - single
- 64  - double

;foo::<16>::float = 2
or
;foo::<16> = 2.0

# Lists ========================
Array
;foo::arr = 5:[1,2,3,4]
- static size

Vector
;foo::vec = [1,2,3,4]
- variable size

Linked list
;foo::llst = [1,2,3,4]
- static size

;foo::lstm = [1,2,3,4]
- var size

# String
a null terminated list of chars, by default ascii encoded
;foo::str = "<onyx> is great!"

for utf-8
;foo::str::utf8 = "Mình nói tiếng Việt"

# Definitions ==================
.def NAME::type, value

will basically replace every occurance of NAME with value
name *Should* be UPPERCASE

#=============================
#||       Maybe Later       ||
#=============================

# Structs
.def NAME::struct, [
    field1::<8>
    field2::<16>
    field3::<64>
    field4::char
    field5::str::utf8
]

# Enums
.def NAME::enum, [
    field1
    field2
    field3
    field4
    field5
    field6
    field7
    field7
]
