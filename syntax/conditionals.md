all math and logic expressions must be wrapped in `()`
```
(1 + 2)   // 3
```

as there's no dedicated bool type, a `false` is a `0` and a `true` is anything above  
those are macro'd to `0` and `1`

negation
```
(1 = 1) => | $puts "math doesn't work"
```

the `=>` operator will execute the right hand side if the left hand side is true
```
(1 = 1) => !$puts "math works"
```

```
(1 = 1) => $puts "math works" | $puts "math doesn't work"
```
