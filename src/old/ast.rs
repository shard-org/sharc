use std::rc::Rc;

pub enum AST {
    Assignment(Rc<AST>, Rc<AST>),
    Add(Rc<AST>, Option<Rc<AST>>),
    Subtract(Rc<AST>, Option<Rc<AST>>),
    Multiply(Rc<AST>, Option<Rc<AST>>),
    Divide(Rc<AST>, Option<Rc<AST>>),
    BitwiseAnd(Rc<AST>, Option<Rc<AST>>),
    BitwiseOr(Rc<AST>, Option<Rc<AST>>),
    BitwiseXor(Rc<AST>, Option<Rc<AST>>),
    BitwiseNot(Rc<AST>, Option<Rc<AST>>),
    BitwiseLeftShift(Rc<AST>, Rc<AST>),
    BitwiseRightShift(Rc<AST>, Rc<AST>),
    Increment(Rc<AST>),
    Decrement(Rc<AST>),
    PopStackInto(Rc<AST>),
    PeekStackInto(Rc<AST>),

}
