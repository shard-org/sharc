this is still in a REALLY REALLY Early stage, like not even usable   

If ye like this concept then PLEASE Help Out!  
I cant do it all by myself... :/  

# General
- `.ox` file extension
- asm but with high level features, like iterators and closures  
- very hands on   
- most things are represented as just series of bits making ye have to think bout how stuff maps in mem  
- Freedom of asm, but without the tediousness and low readability of it
- For now: Compile to asm, and let nasm do the rest  

# Hello World
```
main {
    *stdout <- "Hello, World!"
}
```

# std subroutine example
```
// check if a value is null, or 0... they're the same
nil<&> -> <1> {
    <- (arg1 = 0) 
}
```

# Fibonacci
```
main -> <$0> {
    ;n = 9
    print_fib
}

print_fib -> *stdout {
    (n < 1) => ret<"Invalid Number of Terms!\n">
    (n = 1) => ret<"0\n">
    (n = 2) => ret<"0 1\n">
    
    *stdout <- "0 1\n"

    ;#prev1 = 1
    ;#prev2 = 0

    @loop
    ;fn = (prev1 + prev2)

    prev2 <=?! prev1
    prev1 <=?! fn

    *stdout <- cat<&, "\n"> <- fmt<#fn>
    dec<n>
    (n > 0) => jmp(loop)

    ret<#>
}
```

