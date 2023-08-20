[2023-08-17]

!!!THIS IS SUPPOSED TO BE READ PLAINTEXT!!!

Okay lets shoot the poop

# Includes ==========================================================
I was writing a sketch for the `README.md` and realized we need to have a good way to handle includes

now, there's a few options:
Like C - just Copy-Paste the code wherever the include is
    + Easy to Implement
    + ye can fine-adjust the prescedence of includes
    - Awful for Error Handling
    - Function, Marker, and Subroutine names would have to be unique, and thats a nightmare when using 3rd-party libraries
Like Rust - Modules Modules Namespaces Crates Modules
    + fairly Intuitive
    + a tested solution
    - PAIN to implement
    - doesnt fit the feel we're goin for
    
But uh.. I dont like either. So I came up with a .. Solution..

whenever ye'd compile the project every file within the project dir (and it's subdirs) is automatically included as well. 
If ye wanted to import a library, ye'd have to do so explicitly.
They would be split up into *modules*, this means: the library would have subdirectories and each one of them would be a different module, ye could import them using this syntax:
```
.inc library.module
```
this by default imports a system library, for a custom location wrap this in `""` like this:
```
.inc "path/to/lib".module
```
modules can have modules inside as well! each one would be separated by a `.`

Behind the Hood this approach would first parse the main file, look for files within the project dir, and parse all of them separately
This would be parsed by the preprocessor and would (hopefully) evaluate to something along these lines
```rust
// (filename, contents)
Vec<(String, String)>
```
and for libraries
```rust
// for libraries ==========
struct Lib {
    name: String,
    module: Option<String>,
    contents: String,
}

Option<Vec<Lib>>
```

The contents of each lib would never touch, and they'd be parsed separately
Only when it's time to compile they'd be cross referenced and all used parts would be copy-pasted in the right places, everything unused just wouldn't ever be referenced so the compiler wouldn't add it.

And now we reach that one dreaded topic: **NAMESPACES**
I *obviously* dont want this to be a thing, but, What would happen if ye import a library, the library has functions, markers, all that jazz, right?
and they have names. So if lets say ye wanna import a func `add` from one library, simple right? and yeah it is.
but then lets say ye want to import a function of that same name from a different library. How would that work? ye'd give this to the linker module and it'd just be like
"uhh idk m8 im told to import `add` but there's two idk what ye want me to do"

given that these are libraries, which can be 3rd-party pieces of code throwing an error message wouldn't cut it cause ye arent expected to go edit the lib
so either have stuff be namespaced
```
.inc libA = A
.inc libB

!A.func  // calls from libA
!func    // calls from libB
```
or... idk.
I wanna find better solutions, and I'm pretty sure there is one, but idk

also to adress a question some of ye might have "how does the compiler know where one func ends and another begins?" simple
```
@some_function x -> y
    // do some code
    (x > 40) => ret 2
    ret x

// it ends here, when another one begins, cause..  ye can't have one within the other
@another_func
```

# Macros ==========================================================
I'm talking compile-time built in macros, not an interface to let people create their own, these would be used for just quality of life.
like `$len` for getting the length of a static, or `$elen` for getting the length of each element of an array.
Useful for not having to keep track of stuff in ye head, (even asm has these):
```asm
section .data
    hello     db  "Hello, World!", 10, 0
    hello_len equ $ - hello
```

We're gonna have to have a meeting about syntax and implementation of these, lets talk in #dev-general and figure out a good time

# Calling Func (From Func)
lets take a rust example so we know what we're talking bout
```rust
multipoly( add(6, 18), 1 );
```
simple right? rust and other C styled langs easily parse this cause each function's args are within `()` after it
well.. its not so simple here. 
but that *might* be fine! we might not want to allow for this to allow for more precise mem managment!

so ye lemme show an example
```
!multiply !add 6, 18, 1
```
now ye can figure out how to read this, and the compiler easily can! BUT can you at a quick glance? prob no
this makes code less readable, which is why I'd wanna have some kind of *transparent delimiter*
*transparent* cause it doesnt do anything and is just visual

```
!multiply <!add 6, 18>, 1
```
like this perhaps? still dunno bout the look of `<>` but it's *prob* fine? throw yer ideas in chat
