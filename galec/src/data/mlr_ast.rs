use crate::interpreter::{ Program, Environment };
use crate::data::values::{ Value };
// Medium Level Representation AST


// Function and File cannot appear everywhere

#[derive(Debug)]
pub struct Function {
    pub name: Name,
    pub parameters: Vec<Identifier>,
    pub implementation: FunctionImpl
}

pub enum FunctionImpl {
    Code(Box<Tree>),
    Native(fn(&Program, &mut Environment) -> Value)
}

impl std::fmt::Debug for FunctionImpl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match &self {
            FunctionImpl::Code(t) => write!(f, "Code({:?})", t),
            FunctionImpl::Native(_) => write!(f, "[Native function]")
        }
    }
}

#[derive(Debug)]
pub struct File {
    pub functions: Vec<Function>
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Name {
    pub text: String
}

#[derive(Debug)]
pub struct Identifier {
    pub name: Name
}

#[derive(Debug)]
pub struct Let {
    pub id: Identifier,
    pub exp: Box<Tree>
}

#[derive(Debug)]
pub struct Seq {
    pub elements: Vec<Tree>
}

#[derive(Debug)]
pub struct Apply {
    pub fn_name: Identifier,
    pub param: Box<Tree>
}

#[derive(Debug)]
pub struct BinOp {
    pub lhs: Box<Tree>,
    pub rhs: Box<Tree>,
    pub op_type: BinOpType
}

#[derive(Debug, Copy, Clone)]
pub enum BinOpType {
    Mult,
    Plus,
    ArrIndex
}

#[derive(Debug)]
pub struct Number {
    pub value: i64
}

#[derive(Debug)]
pub struct Boolean {
    pub value: bool
}

#[derive(Debug)]
pub struct Text {
    pub text: String
}

#[derive(Debug)]
pub struct Tuple {
    pub elements: Vec<Tree>
}

#[derive(Debug)]
pub struct Array {
    pub elements: Vec<Tree>
}

#[derive(Debug)]
pub enum Tree {
    Let(Let),
    Seq(Seq),
    Identifier(Identifier),
    BinOp(BinOp),
    Number(Number),
    Boolean(Boolean),
    Text(Text),
    Tuple(Tuple),
    Array(Array),
    File(File),
    Function(Function),
    Apply(Apply)
}

impl Tree {
    pub fn from_text(s: String) -> Tree {
        Tree::Text(Text { text: s })
    }
    pub fn from_array(elems: Vec<Tree>) -> Tree {
        Tree::Array(Array { elements: elems })
    }
}

// Constructors

impl Name {
    pub fn from_string(s: String) -> Name {
        Name { text: s }
    }
    pub fn from_str(s: &str) -> Name {
        Name::from_string(String::from(s))
    }
}

impl Function {
    pub fn from_code(n: Name, p: Vec<Identifier>, b: Box<Tree>) -> Function {
        Function { name: n, parameters: p, implementation: FunctionImpl::Code(b) }
    }
    pub fn from_native(n: Name, p: Vec<Identifier>, b: fn(&Program, &mut Environment) -> Value) -> Function {
        Function { name: n, parameters: p, implementation: FunctionImpl::Native(b) }
    }
}

impl File {
    pub fn new() -> File {
        File{ functions: Vec::new() }
    }
}

impl Identifier {
    pub fn from_name(n: Name) -> Identifier {
        Identifier { name: n }
    }

    pub fn from_string(n: String) -> Identifier {
        Identifier { name: Name { text: n } }
    }

    pub fn from_str(n: &str) -> Identifier {
        Identifier { name: Name { text: String::from(n) } }
    }
}

impl Let {
    pub fn new(id: Identifier, exp: Tree) -> Let {
        Let { id: id, exp: Box::from(exp) }
    }
}

impl Seq {
    pub fn new(stmts: Vec<Tree>) -> Seq {
        Seq { elements: stmts }
    }
}

impl Apply {
    pub fn new(id: Identifier, param: Tree) -> Apply {
        Apply { fn_name: id, param: Box::from(param) }
    }
}

impl BinOp {
    pub fn from(lhs: Tree, rhs: Tree, op_type: BinOpType) -> BinOp {
        BinOp { lhs: Box::from(lhs), rhs: Box::from(rhs), op_type: op_type }
    }
}

impl Number {
    pub fn from(v: i64) -> Number {
        Number { value: v }
    }
}

impl Boolean {
    pub fn from(v: bool) -> Boolean {
        Boolean { value: v }
    }
}

impl Tuple {
    pub fn from(elems: Vec<Tree>) -> Tuple {
        Tuple { elements: elems }
    }
}

