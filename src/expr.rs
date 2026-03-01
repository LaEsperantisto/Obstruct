use crate::env::Environment;
use crate::span::Span;
use crate::type_env::Type;
use crate::value::Value;

#[derive(Debug, Clone)]
pub enum UseKind {
    Normal,
    Std,
}
#[derive(Debug, Clone)]
pub enum Expr {
    Nothing(),
    // Literals
    Float(f64),
    Int(i32),
    Bool(bool),
    Str(String),
    Char(String),
    Vector(Vec<Expr>),
    Array(Vec<Expr>),

    // Binary Operators
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mult(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Mod(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
    EqualEqual(Box<Expr>, Box<Expr>),
    BangEqual(Box<Expr>, Box<Expr>),
    GreaterEqual(Box<Expr>, Box<Expr>),
    LessEqual(Box<Expr>, Box<Expr>),
    Less(Box<Expr>, Box<Expr>),
    Greater(Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),

    Nth(Box<Expr>, Box<Expr>),

    // Unary Operators
    Not(Box<Expr>),

    // Statements
    StmtBlock(Vec<Box<Expr>>),
    StmtBlockNoScope(Vec<Box<Expr>>),
    Print(Box<Expr>),
    Discard(Box<Expr>),

    // Functions
    DeclareFunction(
        String,
        Box<Expr>,
        Type,
        bool,
        Vec<(String, Type)>,
        Vec<String>,
        Span,
    ),
    Function(Box<Expr>, Type, Vec<(String, Type)>, Vec<String>),
    CallFunc(String, Vec<Type>, Vec<Box<Expr>>, Span),
    Return(Box<Expr>),

    // Variables
    Variable(String, Span),
    DeclareAndAssign(String, Box<Expr>, bool),
    Declare(String, Type, bool, Span),
    Assign(String, Box<Expr>, Span),
    Delete(String),
    This(),

    // Control Flow
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // if condition, if block, else block
    While(Box<Expr>, Box<Expr>),
    For(String, Box<Expr>, Box<Expr>, Span), // loopee, looper, block

    // Others
    Custom(fn(&mut Environment) -> Value),
    Custom2(fn(&mut Environment, Vec<Value>) -> Value),
    Value(Value),
    Use {
        kind: UseKind,
        path: String,
        span: Span,
    },
}
