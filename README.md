this is still in a REALLY REALLY Early stage, like not even usable   

If ye like this concept then PLEASE Help Out!  
I cant do it all by myself... :/  

Check out `TUTORIAL.md` for a quick crash course

# General
- `.ox` file extension
- asm but with high level features, like iterators and closures  
- very hands on   
- most things are represented as just series of bits making ye have to think bout how stuff maps in mem  
- Freedom of asm, but without the tediousness and low readability of it
- For now: Compile to asm, and let nasm do the rest  
- Both Curly Brackets, and Significant whitespace

# Done
- Comments `//`
- Parsing Markers
- Console Logger

# Hello World
```
main {
    *stdout <= "Hello, World!\n"
}
```
or
```
@std prtl
main {
    prtl<"Hello, World!">
}
```

# std subroutine examples
```
// `&*` means the subroutine takes any number of arguments of any "type"
@std cat fmt
prtl<&*> {
    // `$&*` means it'll take all of the args 
    *stdout <= cat<&, "\n"> <- fmt<$&*>
}
```
<!-- ``` -->
<!-- // used for indexing linked lists, takes the first element and wanted index -->
<!-- llix<&(ptr::llp):(start), &(8,16,32,64):(index)> -> <&>|!<E> { -->
<!--      -->
<!--     @loop -->
<!--  -->
<!--     // TODO  -->
<!--      -->
<!--     dec<index> -->
<!--     (index > 0) => jmp(loop) -->
<!--  -->
<!-- } -->
<!-- ``` -->

# Fibonacci
```
// exit with code, `$0` specifies that we want a literal number `0`
main -> <$0> {
    ;n = 9
    print_fib
}

// all returns go to the stdout channel
print_fib -> *stdout {
    // `ret<>` returns
    (n < 1) => ret<"Invalid Number of Terms!\n">
    (n = 1) => ret<"0\n">
    (n = 2) => ret<"0 1\n">
    
    *stdout <= "0 1\n"

    // # means the var will be freed at the end of the scope
    ;#prev1 = 1
    ;#prev2 = 0

    @loop
    ;fn = (prev1 + prev2)

    prev2 <= prev1
    prev1 <= fn

    // `#` here explicitly frees the value after its use
    // `fmt<>` formats the integer into ascii, `cat<>` concatinates two strings
    *stdout <- cat<&, "\n"> <- fmt<#fn>

    // decrement by 1
    dec<n>

    // jump to `@loop`
    (n > 0) => jmp(loop)

    // return nothing, (unsafe!)
    ret<#>
}
```

