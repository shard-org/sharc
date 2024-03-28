shard is still at a VERY EARLY STAGE, like not even usable yet  
If you like this concept then PLEASE help out  
I cant do it all by myself.. :/  

Our Discord: https://discord.gg/z3Qnr87e7c  
for **contributing** ^^^   

# Features
- Fine grained low level access to hardware features, like registers, syscalls, interrupts
- No "safety features" preventing you from doing what you want
- Terse, yet clear and concise syntax and language
- Simple macro system allowing for functions as well as "search and replace"
- Customizable compiler verbs and build scripts, all in-language
- Unique features like: *Label Attributes*, *Typed Operators*, and *Anonymous Labels*

# Non-Features
- Do not strife for cross-platform compatibility. As hardware features are exposed to the user code becomes inherently incompatible
- No complex standard library, used mostly for architecture-specific definitions and fixes
- No types beyond byte sizes, pointers, and structs

# Code Examples
(just theoretical for now)

## Hello World
We're using the linux `write` syscall directly here
```
:arch x86_64 linux

entry:
   *write 1, "Hello, World!", 13
   ret
```

## Fibonacci
```
:arch x86_64 linux
:linker #LINKER -lc

fibonacci n 2 -> 2 {
   (n <= 1) ret n
   ret !fibonacci (n - 1) + !fibonacci (n - 2)
}

entry:
   %terms 2
   $scanf "%d", terms

   %i 2
   loop (i < terms) => {'i ++} {
      !fibonacci i
         =>> $printf "%d\n"
   }

   ret
```

## Bubble Sort
The script will run after running `sharc run`.
Values starting with `#` are macro invocations referring to the current project.
```
:name bubble_sort
:arch x86_64 linux
:linker #LINKER -lc

:verb run /bin/sh {
   sharc #FILE
   chmod +x #NAME
   ./#NAME
}

entry:
   %array [2] = (2, 8, 9, 7, 4, 3, 6, 5, 1, 0)

   !bubble_sort array, 10

   // print the array
   %i 2
   loop (i = 10) => {'i ++} {
      printf "%d\n", array.i
   }

   ret

bubble_sort [2] array, 2 len {
   %i 2
   loop (i < len) => {'i ++} {
      %j 2
      loop (j < len-i-1) => {'j ++} {
         (array.j > array.(j+1)) 
            '[array.j] : [array.(j+1)]
      }
   }
}
```

## Python?!
I've realised this system technically lets you convert shard files into python files (or any other language).
```
:verb DEFAULT /bin/sh {
    tail +4 #FILE | python
}

print("Hello, World!")
```
Additionally the interpreter could also be set to `/usr/bin/python` for unlimited crazyness
