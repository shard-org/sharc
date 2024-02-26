# Types

## Sizes
Represtent number of bytes that the data takes up.
eg: `1`, `8`, `98`


## Derivative types
- **Floats:**
    Floating point number. `fN` where N ∈ {4, 8}
    eg: `f4`, `f8`

- **Signed Integers:**
    An integer with a sign bit prepended. `sN` where N ∈ {1, 2, 4, 8}
    eg: `s4`, `s1`


## Composite types
- **Pointers:**
    Size equal to architecture word. Wraps around a type T. `[T]`
    eg: `[8]`, `[s2]`

- **Arrays:**
    List of N elements of type T. `T:N`
    eg: `4:66`, `f4:6`

- **Structs:**
    Hold types and namespaced labels. Referenced by identifier.
    Comp WARN if not CamelCase
    eg: `String`, `Position`



# Labels and Functions

## Labels
Labels are defined as a pointer to wherever they're placed in the final binary.
All statics, constants, functions, loops, etc are essentially labels.

They begin with an identifier and end with `:`.
eg: `main:`


## Calls and Returns
Calling an address pushes the current IP to the stack,
later whenever returning a word is popped from the stack and IP set to the value. 

use the `!` symbol to call an adress and the `ret` keyword to return.
eg: `!print_hello`, `!0xdeadbeef`


## Functions
Functions are a special case of a label containing a body
and enforced arguments and/or return variables.

They may contain labels, jumps, etc. 

To return use the `end` keyword, which takes return arguments
eg: `end 16`, `end array`


# Variables

## static, const, init

## Register vars

## stack vars

## other
