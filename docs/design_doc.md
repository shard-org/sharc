An idea I've had for a rework of the syntax, and a more minor one of how it operates.
Here's two code examples (the same that can be found on the README.md).
Dont Worry I'll explain what they mean soon

## Hello World
```
.use std prtl

@main
    !prtl "Hello, World!"
    *ext 0
```

### No Std
```
@main
    ;out: [a8] = "Hello, World!\0"

    ;i = 0
    @loop
        ;temp = [out + i]
        inc i
    (temp != 0) => #loop

    *stdout i, out

    *ext 0
```

## Breakdown
```
// start of the main subroutine
@main
    // the string that we want to output, null terminated but the std will have "fat pointers"
    // [a8] means "pointer to an array of 8 bit elements", this is only for show and self describing code, it doesnt do anything
    ;out: [a8] = "Hello, World!\0"

    ;i = 0
    @loop
        // iterate over the elements of the array by indexing
        ;temp = [out + i]
        inc i  // increment by one

        // as long as the current character is not null, loop back
        (temp != 0) => #loop

    // the stdout syscall requires the length of the array to be printed
    *stdout i, out

    // exit syscall, error code 0
    *ext 0
```
Indentation is not necessary... its just easier to read with it eh? 
might wanna make the comp throw a warning if its way out of whack

okey so ye seen the new syntax, first up: whatcha think?
second, I'll explain what, how, where, when and why

so I looked back at the old syntax and design and I realized "this aint gonna cut it", why? well:
- the old one was too C like, while not exactly a bad thing, i try my best to avoid that territory
- it was pain to read... dont get me wrong still better than asm, but we can do better than that
- it was too high level, well, a mixed bag at that. the `*stdout` print took a ... string? idk
- nothing made sense, operators were dependant heavily on context, and I still had no idea how to tie it all together
- the "scopes" were not really needed, same as types, or like half of the stuff
- stuff didn't really map well to what happened under the hood, and the lang had a decent limit of how low level ye can go

dont get me wrong! we arent goin back to asm that defeats the point of the lang 
the std will still provide all the functions ye can want, and you'll still be able to do all the stuff as before in similar ways
this is mostly for consistance, readability, and ability to work really low level by default. Turtles all the way down ye know.

and no we arent throwing out functions and stuff, they just got a different syntax
```
@add x, y -> z
    ret (x + y)
```

what is all the new stuff?!
ye theres a few new operators and stuff
read below to find out what they do.. or come up for ideas for the ones I have no clue bout\

Current ideas:
```
inc foo
+foo

dec foo
-foo
```


# Operator Meanings

## ; 
Something's being set, created, etc
```
;foo = 87
```

## @
defines something
```
@main
```

## () 
math and logic operations
```
(1 + 1 = 2) => ret
```

## []
pointer
```
;bar = [foo]  
```

## :
further details on something, like type definition
```
;foo: [a8] = "Get Onyx'd\0"
```

## !
subroutine call
```
!prtl "the thing"
```

## *
syscall, interrupt, or any other architecture specific operation
```
*ext 0
```

## `
multi line code
```
!prtl 
    ` "the thing"
```

## &
idk
```
```

## ^
```
```

## %
```
```

## <=
modifying
```
;foo = 8
foo <= 48
```

## $
idk
```
```

## #
jumps
```
#loop
```

## <>
idk
```
```

## |
```
```

## ~
idk
```
```
