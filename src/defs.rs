//
// message definitions ===============================
pub const HELP: &str = "onyx - Compiler for the <onyx> Programming Language
Usage: onyxc <input_file> [OPTIONS]

Options:
  -h, --help      This Message
  -v, --version   Version Number
  -o, --output    Specify the Output Binary
  -d, --debug     Not needed for Mere Mortals :v
  -t, --noclean   Keep Temp Files
  -C, --nobin     Compile to Assembly";

pub const VERSION: &str = "onyx 0.1.0";

//
// Default Constatnts ===============================
pub const DEFAULT_SYS_LIB: &str = "/usr/share/onyx/";

//
// Other Constatnts ===============================
pub const FILE_EXTENSIONS: &[&str] = ["shd", "shard"];

//
// macro definitions ===============================
#[macro_export]
macro_rules! trust_me {
    ($($content:tt)*) => {{
        unsafe { $($content)* }
    }};
}
