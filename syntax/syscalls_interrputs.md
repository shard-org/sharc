things prefixed with `opt_` are optional

## fs
### common file descriptors
```
0 stdin
1 stdout
2 stderr
```

### read
```
&read fd, buffer_ptr, amount
```

example:
```
&open "file.txt", 0, 0
%buffer = $malloc 1024
&read some_file, [buffer], 1024
```

shorthands:
```
&stdin 
```

### write
```
&write fd, buffer_ptr, amount
```

example:
```
;file = &open "file.txt", 0, 0
&write file, "Hello World!", 12
```

shorthands:
```
&stdout "hello, world!\n", 14
&stderr "hello, world!\n", 14
```

### open
```
&open path, flags, opt_mode  // returns file descriptor (fd)
```

example:
```
;file = &open "file.txt", 0, 0
```

flags:
```
CREAT        1000   // create if it doesn't exist
TRUNC        2000   // truncate file
EXCL         4000   // fail if exists
NOCTTY      10000  // don't make it a controlling terminal

NONBLOCK        4  // non-blocking I/O
APPEND         10  // append (writes guaranteed at the end)
DSYNC       40000  // sync data (but not metadata unless FSYNC)
DIRECTORY  100000  // must be a directory
NOFOLLOW   200000  // don't follow links
LARGEFILE  400000  // allow large files (LFS)
DIRECT    2000000  // direct disk access 
NOATIME   4000000  // don't update atime
CLOEXEC  10000000  // set close_on_exec
```

mode:
```
RDONLY 0
WRONLY 1
RDWR   2
```

### close
```
&close fd
```

example:
```
;file = &open "file.txt", 0, 0
&close file
```


