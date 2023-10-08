this is still in a REALLY REALLY early stage, like not even usable  
if ye like this concept then PLEASE help out  
I cant do it all by myself.. :/  

Our Discord: https://discord.gg/TsMxccfym  
for **contributing** ^^^


# Concept
- keeps the overall feel and spirit of asm with some additional higher level syntax and features 
- jump anywhere, anytime. Making spaghetti code easier than ever
- Nothing Stopping you from doing what you want, no safety nets
- barren type system featuring the bare necessities
- a simple std lib for abstracting away the tediousness
- DENSE operator oriented syntax (or "operator soup" for fans and lovers)

# Disclaimers
- This lang isn't meant to be used for serious projects, it's primarly hobby and fun
- there's gonna be drastic changes throughout the development process
- It's AWFULY unsafe, the registers, stack, and syscalls are all exposed to the user

# Code Examples

## Hello World
```
main:
    $puts "Hello World"
    *exit 0
```

## Fibonacci
```
main:
    %n 2 = 9
    (n < 1) => $puts "Invalid Number of Terms!\0"
    (n = 1) => $puts "0\0"
    (n = 2) => $puts "0 1\0"

    %arg1 2 = 1
    %arg2 2 = 0

loop:
    ;temp r3 = ([arg1] + [arg2])

    'arg2 : arg1
    'arg1 : temp

    $printf "%d\n\0", temp 
    'n --
    (n > 0) => jmp loop

    $puts "Done!\0"
    ret
```

