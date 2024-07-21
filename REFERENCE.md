# Code Style
> Fully qualify paths if an object or module are used only once in a file.  
<br>
  
> Don't merge all imports from a library into one `use`; eg:
> ```
> use std::{
>     fs,
>     io::{self, Read, Write},
>     collections::{HashMap, VecDeque},
> };
> ```
> Instead, split these into multiple.
> ```
> use std::fs;
> use std::io::{self, Read, Write};
> use std::collections::{HashMap, VecDeque};
> ```
<br>

> run `rustfmt` :)

> If a lifetime must annotate more than one field/var, it should likely be expended to a more useful name;
> Don't call them a 1 letter long name like `'a`.  
> The impl block should use the same lifetime names as the object. Don't shorten them.

> Trivial lifetimes *may* be described using a single letter, however try avoiding using an alphabetic order for lifetimes
> `'a`, `'b`, `'c`. Instead use a letter 


# Exit Codes
Exit Codes for `sharc` should vary depending on where the error appeared

|Exit Code  |  Reason      |
|-----------|--------------|
|    1      |  generic     |
|    2      |  arg parsing |
|    3      |  file io     |
|    9      |  lexer       |
|    16     |  parser      |
|    22     |  macros      |
|    28     |  codegen     |
|    69     |  easter eggs |
