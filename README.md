shard is still in a VERY EARLY STAGE, like not even usable yet  
If you like this concept then PLEASE help out  
I cant do it all by myself.. :/  

Our Discord: https://discord.gg/z3Qnr87e7c  
for **contributing** ^^^   

We've also got a website now! https://shardlang.org/  
Although it does need full rework.. Hey any frontend devs?  

# Features
- Fine grained low level access to hardware features, like registers, syscalls, interrupts
- No "safety features" preventing you from doing what you want
- Keep the syntax and language terse, yet clear and concise
- Simple macro system allowing for functions as well as "search and replace"
- Customizable compiler verbs and build scripts, all in-language

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

main entry:
   *write 1, "Hello, World!", 13
   ret
```

## Fibonacci
```
:arch x86_64 linux
:link -lc

fibonacci n 2 -> 2 {
   n <= 1 -> end n
   end @fibonacci (n - 1) + @fibonacci (n - 2)
}

main entry:
   %terms 2 = 9

   %i 2 = 0
   loop i < terms {
      $printf "%d ", @fibonacci i
      'i ++
   }

   ret
```

## Bubble Sort
The script will run after running `sharc run`.
Values starting with `#` are macro invocations referring to the current project.
```
:name bubble_sort
:arch x86_64 linux
:link -lc

:verb run /bin/sh {
   sharc #FILE
   chmod +x #NAME
   ./#NAME
}

main entry:
   %array [2] = { 7, 5, 1, 4, 9, 8, 2, 6, 3 }
   %length 2 = 9

   bubble_sort array, length

   loop length > 0 {
      'length --
      $printf "%d", array.length
   }

   ret

bubble_sort [2] array, 2 n {
   %i 2 = 0
   loop i < n {
      %j 2 = 0
      loop j < n-i-1 {
         (array.j > array.(j+1))
            '[array.j] : [array.(j+1)]
         'j ++
      } 
      'i ++
   }
}
```

## Python?!
I've realised this system technically lets you run python (or any other language) from `sharc`.
```
:verb py /bin/sh {
    tail +4 #FILE | python
}

print("Hello, World!")
```
Additionally the interpreter could also be set to `/usr/bin/python` for unlimited crazyness
