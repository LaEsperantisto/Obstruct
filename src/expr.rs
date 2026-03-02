use crate::runtime_env::RuntimeEnvironment;
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
    Add(Box<Expr>, Box<Expr>, Span),
    Sub(Box<Expr>, Box<Expr>, Span),
    Mult(Box<Expr>, Box<Expr>, Span),
    Div(Box<Expr>, Box<Expr>, Span),
    Mod(Box<Expr>, Box<Expr>, Span),
    Power(Box<Expr>, Box<Expr>, Span),
    EqualEqual(Box<Expr>, Box<Expr>, Span),
    BangEqual(Box<Expr>, Box<Expr>, Span),
    GreaterEqual(Box<Expr>, Box<Expr>, Span),
    LessEqual(Box<Expr>, Box<Expr>, Span),
    Less(Box<Expr>, Box<Expr>, Span),
    Greater(Box<Expr>, Box<Expr>, Span),
    And(Box<Expr>, Box<Expr>, Span),
    Or(Box<Expr>, Box<Expr>, Span),

    Nth(Box<Expr>, Box<Expr>, Span),

    // Unary Operators
    Not(Box<Expr>),

    // Statements
    StmtBlock(Vec<Box<Expr>>),
    StmtBlockNoScope(Vec<Box<Expr>>),
    Print(Box<Expr>, Span),
    Discard(Box<Expr>),
    Stmt(Box<Expr>),

    // Functions
    DeclareFunction(
        String,
        Box<Expr>,
        Option<Type>,
        Vec<(String, Type)>,
        Vec<String>,
        Span,
    ),
    Function(Box<Expr>, Type, Vec<(String, Type)>, Vec<String>),
    CallFunc(String, Vec<Type>, Vec<Box<Expr>>, Span),
    Return(Box<Expr>),

    // Variables
    Variable(String, Span),
    Declare(String, Option<Type>, Option<Box<Expr>>, bool, Span),
    Assign(String, Box<Expr>, Span),
    Delete(String),
    This(),

    // Control Flow
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // if condition, if block, else block
    While(Box<Expr>, Box<Expr>),
    For(String, Box<Expr>, Box<Expr>, Span), // loopee, looper, block

    // Others
    Custom(fn(&mut RuntimeEnvironment) -> Value),
    Custom2(fn(&mut RuntimeEnvironment, Vec<Value>) -> Value),
    Value(Value),
    Use {
        kind: UseKind,
        path: String,
        span: Span,
    },
}
