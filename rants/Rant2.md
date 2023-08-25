[2023-08-23]

!!!THIS IS SUPPOSED TO BE READ PLAINTEXT!!!

ok lets go
# Name =====================================================================
hope ye noticed, but we're switchin the name to `shard` cause `onyxlang.org` is taken, and no idea what icon to get for it.
do ye like it? got any better ideas? lemme know!

file extension wise there's a vote in #announcements so far `.shd` is winnin

# Statics n Constants ======================================================
constants are simple as, cause they're just a compile time copy paste:
```
.def ten 10

(ten + 2) // 12
```

statics uh, i guess could do it like this:
```
;addnum = 20 // static var, cause its within the header

@main
    ;foo = 10  // normal var

    (foo + addnum) // 30

    addnum <= 30   // mutate

    (foo + addnum) // 40
```

# More syntax Clarifications ===============================================
```
// this is the HEADER
.def ten 10
;foo = 8

.inc std.io

// ENTRANCE POINT
@main
    ;bar = 10
    ;baz = 20

    ;foo = @add bar, baz

// MAIN END
// FUNCTION START
@add x1, y1 -> 1
    ret (x + y)
```

# Sized Definitons =========================================================
basically how in rust ye gotta provide a type for function definitions n stuff
here we dont have types, the closest we've got is sizes which are as follows:
- 1 - 8 bit
- 2 - 16 bit
- 4 - 32 bit
- 8 - 64 bit

even asm has this with `byte`,`word`,`dword`, and`qword`

this could be dropped on 8bit systems cause everythin's 8bit
we could make this optional, where the compiler could just figure out based on the size of the value

## syntax
4 byte var:
```
;foo 4 = 10
```

stack allocated array:
```
;foo 1 = 23, 45, 67, 89, 10, 11, 12, 13
```

function definition:
```
@add x 2, y 2 -> 2
    ret (x + y)
```


another idea: a macro to simplify indexing an array
```
;foo 2 = 23, 45, 67, 89, 10, 11, 12, 13
$inx foo, 3 // 89
```
where normally you'd have to do:
```
;foo 2 = 23, 45, 67, 89, 10, 11, 12, 13
[foo + 3 * 2] // where 2 is the size of element
```

also need a name for it `$inx` could work I guess, `$i` is shorter, idk

# Indexing Arrays ==========================================================
currently it's just by adding/subtracting to a pointer (WHICH I LIKE),
but its also kinda a pain with stack allocated stuff, and arrays where each element is more than a byte

on the heap:
```
;array 1 = 17, 28, 3, 78
;element_two 1 = [array + 1]


;array 4 = 17, 28, 3, 78 
;element_two 4 = [array + 4]
```

stack:
```
;array 1 = 17, 28, 3, 78
;element_two 1 = [array - 2]


;array 4 = 17, 28, 3, 78
;element_two 4 = [array - 12]
```

values on the stack are stored in reverse order which hurts a lot, so we've gotta have a way to make this a bit less painful
maybe a macro?
```
;array 1 = 17, 28, 3, 78    
;element_two 1 = [array + $inx 1]

;array 4 = 17, 28, 3, 78
;element_two 4 = [array + $inx 1]
```


# Reserving Memory =========================================================
create an empty var, which reserves the mem needed:
```
;foo 4 // this reserves 4 bytes on the stack
;foo 4 = $emp 5 // creates 5 empty elements of 4 bytes each
```
would also need a way to reserve n bytes/elements in an array, no idea what the syntax would look like

**LATER:**
- dynamic allocation
- freeing memory

# System Libraries =========================================================
we know em, we love em, but we need a place to put em.
there's already an env var `SHARD_LIB_PATH`, but there's also gotta be a default.
`/usr/share/shard/` ??? maybe? idk

# Multiple Returns =========================================================
so this isn't the union type, the variables arent stored together in mem, they're fully independant and separate.
the questions are:
- is this a good idea?
- what kind of syntax should be used?

1st one I leave to ya.. It's prob useful in a lot of cases, and its not hard to implement but idk

so here's the syntax I'm thinking of:
```
@main 
    ;foo, bar = @add 16, 87
    // foo = 17
    // bar = 88

@add x1, y1 -> 1, 1
    inc x
    inc y
    ret x, y
```
