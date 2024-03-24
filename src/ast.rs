Type Ident = Box<str>;
Type Body = Vec<Node>;

Type FileRoot = Vec<Node>;

pub enum Node {
    Body(<Vec<Node>),

    Label(Option<Ident>, Vec<LabelAttr>, Option<Body>),
    MacroDef(Ident, Node), // /HELLO "hello, world"
    Tag(Tag),

    Jmp(Expr),
    Ret(Option<Expr>),
    
    StackAssign(Ident, TypedOp, Expr),
    RegAssign(Ident, Reg, Expr),

    Mutate(Ident, TypedOp, Expr),

    Struct(Ident, Vec<(Type, Ident)>),
}


/*
 * Expressions
 */
pub enum Expr {
    Lit(Literal),
    Ident(Ident),
    MacroInv(Ident),       // #HELLO

    ExternCall(Ident, Vec<Expr>),
    FuncCall(Ident, Vec<Expr>), // !print "hello", "world"
    // Interrupt(),

    Unary(Expr, TypedOp),
    Binary(Expr, TypedOp, Expr),
    
    IndexOf(Expr, Expr),
    FieldOf(Expr, Ident),
}


/*
 * Literals
 */
pub enum Literal {
    Char(char),
    Str(String),

    Int(usize),
    Float(f64),
    SInt(isize),

    Array(Vec<Expr>),
}


/*
 * Operators
 */
pub type TypedOp = (Option<Type>, OpKind);
enum OpKind {
    BitAnd, // &
    BitOr,  // |
    BitXor, // ^

    LogAnd, // &&
    LogOr,  // ||
    BitXor, // ^^

    Add,    // +
    Sub,    // -
    Mul,    // *
    Div,    // /

    RShift, // >>
    LShift, // <<

    Eq,     // =

    Inc,    // ++
    Dec,    // --
}


/*
 * Label
 */
pub enum LabelAttr {
    Loop,
    Entry,
    Inline,
    Export,

    Org, // const = @ .rodata

    Cond(Expr),

    Arg(Ident, Type),
    Return(Type),
    PtrType(Type),
}


/*
 * Types
 */
Type Size = usize;
pub enum Type {
    Size(Size),   // 8

    Float(Size),  // f4
    SInt(Size),   // s2

    Array(Type, usize), // s8:40
    Ptr(Type),          // [1]
    Struct(Ident),      // MyStruct

    Void,

    Reg(Reg),
}

struct Reg { // register
    id: usize,
    size: Size,  // q, d, l, s;  w -> S_ArchWord
}


/*
 * TAGS
 */
enum Tag {
    Name(Ident), // :name hello_world
    Arch(Ident), // :arch x86_64 linux
    Version(usize), // :version 0.4 -> 04
    
    NoSTD, // :nostd
    Dep(Ident), // pulled from repos
    Use(Ident), // other shard files
    Lib(Ident), // extern libs?
    
    Linker(),
    Assembler(),
    Sharc(),
    
    // INTERNAL

}

// // build components
// pub linker:    Exec, // required!
// pub assembler: Exec, // required!
// pub sharc:     Vec<String>,
//
// pub verbs:     Vec<VerbDef>,
//
// //
// // architecture specific defs, not *meant* to be used in main files
// // these should be all caps
// pub word:       usize, // word size
// pub attributes: Vec<(String, String)>, // (from, into)
// pub registers:  Vec<(String, String)>, // (from, into)
//
// pub syscall_addr: usize,
// pub syscall_conv: (Vec<String>, String),
// pub syscalls:     Vec<(String, Vec<SysArg>)>,
