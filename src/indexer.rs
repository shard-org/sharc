pub type Index = Vec<FileIndex>;

pub struct FileIndex {
    pub path: Rc<Path>,
    pub contents: String,
    pub functions: Vec<Func>,
    pub statics: Vec<Static>,
    pub markers: Vec<String>,
}

// only reason it starts with a B is casue rust requires it
pub enum Size {
    B1,  // 8b
    B2,  // 16b
    B4,  // 32b
    B8,  // 64b
    Ptr, // arch dependent
}

//
// functions
pub struct Func {
    pub name: String,
    pub arg:  Option<Vec<Var>>,
    pub ret:  Option<Vec<Var>>,
    pub body: String,
}

pub struct Var {
    pub name: String,
    pub size: Size,
}

pub struct Static {
    pub name: String,
    pub size: Size,
    pub value: String,
}

//
// actual indexer
pub fn indexer(files: Vec<Group>) {
}
