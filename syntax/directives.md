# def - define
defines a variable-like macro, this is a literal copy-paste
```
.def THIRTEEN 13

(1 + THIRTEEN) // 14
```
the last line evaluates to
```
(1 + 13) // 14
```

# ent - entry
the entry point of the program, the compiler will start execution at the label specified
```
.ent start

start: // execution starts here
```
by default the entry point is `main`


# inc - include
include a file or a library.

```
.inc "some_file.shd"   // some_file.shd from the current directory
.inc "/some_file.shd"  // some_file.shd from the root directory
.inc std.io          // the io module from the library `std` from the system library directory
.inc std.io.prtl     // the prtl function from the io module from the std library
```


# con - constant
a read only variable integrated into the final binary, gives a pointer
```
.con TEN 4 = 10 

(1 + [TEN]) // 11
```


# dat - data
basically a static var
```
.dat FOO 8 = 20
```

it doesnt need a value, in which case it will be initialized to null  
```
.dat foo 8
```


# .str - struct 
its a struct
```
.str Foo {
    8 bar  // 8 bytes
    1 baz  // 1 byte 
    4 fiz  // 4 bytes
}

main:
%some_var ? = Foo 
'[some_var.bar] = 20
'[some_var.baz] = 1
'[some_var.fiz] = 10
```

to access diff fields use `.` like indexing arrays


# .txt - text
inserts text into the binary. This is done through creating a constant but ignoring the pointer to it.
```
.txt "Hello World!"
```

# .mac - macro
evaluated at compiletime
```
.mac add x, y {
    (x + y)
}

/add 1, 2   // 3
```
