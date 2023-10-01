A function is an abstract self-contained block of code with a defined scope, name, beginning, and end.  
to preserve the sequential execution core pillar functions must be defined before any label in the program.

this also means that you CAN have a "function file" akin to C's header files but just for functions

to define one use:
```
@hello {
    $puts "Hello, World!"
}
```

they may also accept arguments.. well that's the reason ye'd use one in the first place :p  
the args must always have a size
```
@add x 2, y 2 -> 2 {
    end (x + y)     // return x + y
}
```
`end` returns the value given from the function

keep in mind this is the ONLY exception to the rule of sequential execution, the compiler will not execute the function until it is called.  

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
    end (x + y)
}
```

## attributes

### inline
the inline attribute will cause the compiler to replace the function call with the function body, this is useful for small functions that are called often.

### macro
converts the function into a compiletime macro. use `/` to call it

### ignore 
ignores that it's unused
