# Built-in
## Variables
```
;x = 42      // create var x of the value 42
;x = 0x2A    // also works with hex
;x = 101010b // and binary

// my default 8 bits are used, but you can also specify the length
;x:<16> = 42  // here its 16 bits

```

## Operators
```


```

# Standard Library (std)
## Lists
```
// Sized List, fixed size of each element and the list itself
// the size of each element may be changed, but this would require moving each element's adress and is usually unsafe
;x::sls:<16> = $[1, 2, 3, 6, 2]

// access the fields by using the `inx` subroutine
;number:<8> = inx(sls)<1>
(number = 2)

// a string can be represented as a Sized List of 8 bit elements
;x::sls:<8> = $['B', 'a', 'n', 'a', 'n', 'a']

// there's also a specific type for ascii strings, this assumes the size of each element to be 8b
// allowing for lower memory usage
;x::str = "Banana"

// Unsized List, may have different types and size of elements, element size or count cannot change
;x::uls:<8, sstr> = $[14, "apple"]

// Dynamic Sized List, fixed size of each element, but the list size may change
;x::dls:<16, 5> = $[1, 2, 3, 5]

// Byte Stream, fixed size list of any size of element, no known element boundaries
;x::bst = $[1, 'k', "orange"]


// Subroutines for Lists

```

# Unicode Library (utf8)
```

```
