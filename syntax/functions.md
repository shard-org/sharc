A function is an abstract self-contained block of code with a defined scope, name, beginning, and end.  
DEVNOTE: This is an EXCEPTION to one of the core pillars (one that still doesnt sit right with me), do not use this as a precedent for other things.

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
    <- x + y     // return x + y
}
```

keep in mind this is the ONLY exception to the rule of sequential execution, the compiler will not execute the function until it is called.  

to call one use:
```
*hello
// or
*add 1, 2
```
