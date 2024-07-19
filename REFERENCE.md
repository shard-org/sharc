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

> Don't use 1 letter long lifetime names like `'a`.  
> Give them a name related to what the lifetime is tied to like `'contents`

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
