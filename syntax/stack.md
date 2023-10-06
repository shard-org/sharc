variables here must have a known size at compiletime.  

reserve 4 bytes on the stack and move `20` in there
```
%foo 4 = 20
%foo ? = 20  // same thing, but the size is inferred from the value
```

unlike registers, `foo` isn't the actual value but an offset from the stack base pointer.
you need to dereference it to get the value
```
$puts [foo]   // 20
```

to just reserve the space without assigning a var:
```
%foo 4       // for 1, 2, 4, 8 byte vars
%foo 2:8     // for arrays, 2 elements, 8 bytes
'rsb + 50     // move the stack pointer by 50 bytes
```

stack operations:
```
^20 8    // push 20, size 8
'r1 _    // pop into r1
'r1 ?    // peek into r1
```

ok the first line was a lie, you *can* allocate unknown size vars to the stack. Not saying you *should* but you *can*.   
Let me make this *very* clear, you will need to manually free the memory afterwards, the compiler won't do it for you.   
