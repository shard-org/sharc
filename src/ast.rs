Type Ident = String;

Type FileRoot = Vec<Node>;

pub enum Node {
    Body(<Vec<Node>),

    Label(Ident, Vec<LabelAttr>, Option<Body>),
    MacroDef(Ident, Node), // /HELLO "hello, world"
    
    // Tag(Ident, , Node),     //

    MacroInv(Ident),

    FuncCall(Ident, Vec<Node>),
    ExternCall(),

    Interrupt(),
    
}

pub enum Expr {
    
}


pub enum LabelAttr {
    Entry,
    Inline,
    Arg(Type),
    Return(Type),
}


Type Size = usize;
pub enum Type {
    Size(Size),   // 8

    Float(Size),  // f4
    SInt(Size),   // s2

    Array(Type, usize), // s8:40
    Ptr(Type),          // [1]
    Struct(Ident),      // MyStruct

    Void,
}
