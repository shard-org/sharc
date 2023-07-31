pub static TEMP_ASM: &str = "temp.asm";
pub static TEMP_OBJ: &str = "temp.o";

// TODO: come up with a better name for the compiler. Suggestions so far:
// - Nail
// - Crag
pub static HELP_MESSAGE: &str = 
"onyx - Compiler for the <onyx> Programming Language
Usage: onyxc <IN_FILE> [OPTIONS]

Options:
  -h, --help      Shows this Message
  -v, --version   Prints Version Number
  -o, --output    Specify the Output Binary
  -d, --debug     Not needed for Mere Mortals :v
  -t, --temp      Keep Temp Files
  -C, --noasm     Compile Only, Outputting an Assembly File";
pub static VERSION: &str = "0.0.1";

pub static TIPS: &[&str] = &[
"This Language has Significant Whitespace",
"Use snake_case or ANGRY_SNAKE_CASE",
"Check the Docs Sometime",
"All Files must be within the Working Directory"];

pub static BUILTINS: &[&str] = &[
"inc",
"dec",
"ret",
"jmp"];

pub static STD: &[&str] = &[
"fmt",
"cat",
"prtl"];
