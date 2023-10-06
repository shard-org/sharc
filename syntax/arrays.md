creating an array gives the pointer to the first element, cannot store them in registers  

to create one, either:
```
%bar 1:12 = "hello world"   // 12 byte array from string
%bar 4:2 = {1, 2, 4, 5}    // 4 element array from arbitrary data, 2 byte each
```

the first number is the num of elements, the second is the size of each element
the num of elements may be ommited to represent arbitrary continous data
```
// like this:
%bar :68 = 0*     // 4 element array from arbitrary data, 1 byte each
```

`0*` means "fill with zeros", so each byte will be set to 0

```
[array.0]   // first element
[array.1]   // second element
[array.2]   // third element
```
