creating an array gives the pointer to the first element, cannot store them in registers  

to create one, either:
```
%bar 1:12 = "hello world"   // 12 byte array from string
%bar 2:4 = {1, 2, 4, 5}    // 4 element array from arbitrary data, 2 byte each
```

```
[array.0]   // first element
[array.1]   // second element
[array.2]   // third element
```
