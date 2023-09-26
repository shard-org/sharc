There's no allocator implementation, use whichever you want.
here's an example with `malloc()` and `free()`:
```
;ptr = malloc 4

$printf "ptr: %h" ptr

free ptr
```
