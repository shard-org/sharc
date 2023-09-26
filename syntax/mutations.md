changes the 1st arg (foo), optionally using the 2nd arg (bar) as a value

```
'foo = 20    // set foo to 20
'foo + 20    // add 20 to foo
'foo - 20    // subtract 20 from foo
'foo ^ bar   // xor foo with bar
'foo & bar   // and foo with bar 
'foo | bar   // or foo with bar 
'foo : bar   // same as '[foo] = [bar]
'foo < 20    // shift foo left by 20
'foo > 20    // shift foo right by 20
'foo ! bar   // not bar and store in foo
'foo ++      // increment foo
'foo --      // decrement foo
'foo _       // pop stack into foo
'foo ?       // peek stack into foo
```

in most of these operations the second value is optional, in which case it will use the first value as the second as well.  
```
'foo + 20    // add 20 to foo
'foo +       // add foo to foo
```
