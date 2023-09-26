this is still in a REALLY REALLY early stage, like not even usable  
if ye like this concept then PLEASE help out  
I cant do it all by myself.. :/  

Our Discord: https://discord.gg/TsMxccfym

for **contibuting** check out #2 for the todo list  

# Concept
- keeps the overall feel and spirit of asm with some additional higher level syntax and features 
- jump anywhere, anytime. Making spaghetti code easier than ever
- keep types in your head. Add an integer to a pointer? yupp, no problem. Dereference a float? sure, why not
- a simple std lib for abstracting away the tediousness
- DENSE operator oriented syntax (or "operator soup" for fans and lovers)

# Disclaimers
- This lang isn't meant to be used for serious projects, it's primarly hobby and fun
- there's gonna be drastic changes throughout the development process
- It's awfuly unsafe, there's nothing stopping you from doing stupid shit
- it's not cross platform, merely cross-compatible. So you can write code for any platform, but not compile the same one for multiple.

# Code Examples

## Hello World
```
@main
&prt "Hello World!"
*ext 0
```

## Fibonacci
```
.ent main

@main
!$puts <- !fib 9
*ext 0

@fib n -> msg
(n < 1) => ret "Invalid Number of Terms!"
(n = 1) => ret "0"
(n = 2) => ret "0 1"

%arg1 2 = 1
%arg2 2 = 0

@loop
;temp = (arg1 + arg2)

'arg2 : arg1
'arg1 : temp

!$printf "%d\n", temp 
'n --
(n > 0) => #loop

ret "Done!"
```
