# Newlang example
// default code entrance point
// use `.ent <FUNCTION>` to change that
main {
    // ; is used to create a var
    // by default all are global, use #NAME to free the mem or `#ALL` to free all
    // this value is 10 hex, or 16
    ;foo = 0x10

    // statics may be defined anywhere with ;! , and used anywhere
    // as the name suggests, they're unchanging
    // by default all character lists are ascii
    // if we want the error returned by a subroutine to also be sent, we can use the ?$ operatior
    // statics may altghough take a subroutine as input
    // the only restriction is that they can never change
    ;!fizz = "boop" <- error_code<20>?$

    // vars can also infer size from the largest sized subroutine return
    // the !=> operator is used for running subroutines if the other one returned an error
    // if used within a var assignment the other subroutines must return a valid var
    // here we just return a 0 on fail
    ;added = add_unsafe<25, 87> !=> 0

    // modify variables with the `mod<>` subroutuine, or using the <= operator 
    mod<*added, 2>?!
    *added <=?! 2
    // `mod<>` can accept many inputs and `<=` accepts only one

    //we may also perform logic operations on the results of a subroutine
    // ! after a function means we want to return it instead of the original value
    *added <=?! add<added, 2> 
        => #[<8(x)> -> (x > 8) => ret<8> -> <8>]! 

    // vars must be freed before they're overwritten
    #added

    //use ^ for pushing a value to the stack
    foo^

    // and [n]% for popping the stack, n singifies the place (from top) on the stack,
    // the first one by default. `%^` means it will be popped, used, and then pushed back
    mod<*foo, %>?!

    // ;# means that the var will be freed the nth time it is called, 1 by default
    ;#added = add<1&,2&>
        2<- add_unsafe<2,4> => 0 
        1<- rm(end::8)<&> <- add<1,0>

    prt_add_one_hex<added>

    ;i = 0

    // @ signifies a reference point, you can later jump to one using jmp(POINT)
    @loop

    // use *VAR to modify it
    // modifying always returns an error, if we know we will never overflow we can just use ?!
    mod<*i, &>?! <- (i + 1)?!

    // this may also be written as
    inc<*i>?!
    // where inc increments i by one

    *stdout <- i

    // () signifies it's a logic statement, returning 1 if true
    // skp skips the next subroutine if it's given 1
    skp<(i =< 20)>
    jmp(loop)

    // this can also be done like this
    // => (without a bang) simply runs the subroutine after it if the one on the left returns 1
    (i < 20) => jmp(loop)

    // we could also use an iterator to do this
    // this runs the code within 20 times
    // if we had an array we could've used len<list> which returns the num of elements
    // len by default returns 8 bits, use len(BITS)<> to specify
    // use >> for passing iterators around
    // an iter of an int normally returns an index, we dont have to use that
    iter(20) >> #[*stdout <- fmt(ascii)<&> !=> _ apply<#[<8(x)> -> x * 2 -> <18>]> !=> _]

    // closures let you create ananonymous functions, #[]
    // returning _ causes the subroutine to do nothing
    // indentation may be used to continue the previous line
    *stdout <- fmt(ascii)<&> !=> _ 
        <- apply<#[<8(x)> -> (x * 2) -> <16>]> !=> _ /


    // this exits the program with an exit code 0
    ext<0>
    // you can configure a func to automatically return error codes like this:
    // $ is for specifying "literally NUMBER" instead of the compiler threating that as a bit length
    func<...> -> <$0>|!<$1> 
} 

// takes 2 sets of 8 bits, called x and y (optional, if name not given its just arg[n])
// also expects a var `foo` to be set when its called
// returns one set of 16, and also returns another set of 16 that is
add<2:8(x,y)> -> <16> -> !<on_err> {
    // check if a var foo exists, and is 8 bit
    exp<foo::8>?
    // `<-` at the beginning of a line returns
    <- x + y
}

// this subroutine returns either <8> or an error
add_unsafe<2:8> -> <8>|!<E> {
    <- arg1 + arg2?
}

// `!<on_err>` means that if a subroutine with a ? fails, it will call a subroutine `on_err`
prt_add_one_hex<8> -> !<on_err> {
    // using fmt to format things into ascii, here as a base 10 integer
    // () specifies a sub-sub-routine
    // by default integers are formatted to base 10 use `int::BASE` to change that
    // &[n] represents that the input should be what is given to the func from another one
    // this subroutine can fail given that we're adding 1 to 16 bits, so the ? operator is used to refer to the on_err header
    *stdout <- fmt(u16::ascii)<&>? <- (arg + 1)
}

// subroutines may directly return the result somewhere
on_err<E> -> *stderr {
    // the ?! operator aborts the program if the subroutine fails
    // this checks if all bytes are valid ascii
    exp(ascii)<E>?!

    // removes the last 8 bits (Byte) of a piece of data
    // here we do this so we can concatinate a string to the error
    rm(end::8)<E>?!

    // concatinates two strings of bits
    <- cat<E, &> <- ", called at `on_err`"
}

// like closures in rust, subroutines may accept other subroutines as input
// the F is only here so markdown doesnt treat it as a tag, noramlly S would be used
// calls the function given to it with 5, and 6
apply<F<8>> -> <8>|!<E> {
    <- F<5>
}

error_code<8> -> <8>|!<8> {
    // use () to signify builtin types?? dunno if its called types, but ye get the drill
    // this checks if none of the args are null. * iterates over the entire list.
    // ret may be used to return from a subroutine
    // keep in mind this is heavily unsafe, as ret doesn't need to return any value
    // use # if no value is returned
    exp(Null)<arg*> => ret<#>

    // not only E can be returned as err
    // use $[] to pick the value returned
    ;bar = (200 + 200)?$[1]
    
    // errors also may be handled this way
    <- bar + 1 !=> 1 
}


THE GOAL FOR NOW
- creating vars 
- adding two vars 
- printing 
- returning


```
main -> <$0> {
    ;foo = 24
    ;bar = 42

    ;baz = (foo + bar)
    *stdout <= fmt<baz>
}
```
