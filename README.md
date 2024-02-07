this is still in a REALLY REALLY early stage, like not even usable  
if ye like this concept then PLEASE help out  
I cant do it all by myself.. :/  

Our Discord: https://discord.gg/z3Qnr87e7c  
for **contributing** ^^^   

We've also got a website now! (outdated, unused, needs rework)
https://shardlang.org/ 


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
- It's AWFULY unsafe. Registers, Stack, and Syscalls are all directly exposed.

# Code Examples
(just theoritical for now)

## Hello World
```
main:
    $puts "Hello World"
    ret
```

## Fibonacci
(with libc)
```
entry main
fibonacci n 2 -> 2 {
    (n <= 1) end n
    end @fibonacci (n - 1) + @fibonacci (n - 2)
}

main:
    %n 2 = 9  // num of terms to print

    %i 2 = 0
    loop (i < n) {
        $printf "%d ", (@fibonacci i)
    } then 'i ++

    ret
```

