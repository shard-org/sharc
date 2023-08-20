this is still in a REALLY REALLY early stage, like not even usable  
if ye like this concept then PLEASE help out  
I cant do it all by myself.. :/  

Our Discord: 

for **contibuting** check out #2 for the todo list  

# Concept
- keeps the overall feel and spirit of asm with some additional higher level syntax and features 
- no bs design patterns, oop, or type systems, just code
- keep types in your head. Add an integer to a pointer? yupp
- a simple std lib for abstracting away the tediousness
- DENSE operator oriented syntax

# Code Examples

## Hello World
```
@main
    $prt "Hello World!"
    *ext 0
```

## Fibonacci
```
.sinc std.io

@main
    !prtl <!fib 9>
    *ext 0

@fib n -> msg
    (n < 1) => ret "Invalid Number of Terms!"
    (n = 1) => ret "0"
    (n = 2) => ret "0 1"

    ;arg1 = 1
    ;arg2 = 0

    @loop
        ;temp = (arg1 + arg2)

        arg2 <= arg1
        arg1 <= temp
        
        !prtl <!fmt temp> 
        dec n
    (n > 0) => #loop

    ret "Done!"
```
