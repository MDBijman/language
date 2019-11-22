use std::fmt;

// High Level Representation AST

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Name {
    pub text: String
}

impl Name {
    pub fn from(s: &str) -> Name {
        Name { text: String::from(s) }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub name: Name
}

#[derive(Debug, Clone)]
pub struct Number {
    pub value: i64
}

impl Number {
    pub fn from(v: i64) -> Number {
        Number { value: v }
    }
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool
}

impl Boolean {
    pub fn from(v: bool) -> Boolean {
        Boolean { value: v }
    }
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String
}

#[derive(Debug)]
pub struct Tuple {
    pub elements: Vec<Tree>
}

impl Tuple {
    pub fn from(elems: Vec<Tree>) -> Tuple {
        Tuple { elements: elems }
    }
}

#[derive(Debug)]
pub struct Array {
    pub elements: Vec<Tree>
}

impl Array {
    pub fn from(elems: Vec<Tree>) -> Array {
        Array { elements: elems }
    }
}

#[derive(Debug)]
pub struct Block {
    pub statements: Vec<Tree>
}

impl Block {
    pub fn from(stmts: Vec<Tree>) -> Block {
        Block { statements: stmts }
    }
}

#[derive(Debug)]
pub struct Lambda {
    pub parameters: Vec<Identifier>,
    pub body: Box<Tree>
}

#[derive(Debug)]
pub struct App {
    pub fn_exp: Box<Tree>,
    pub param_exp: Box<Tree>
}

#[derive(Debug)]
pub struct File {
    pub statements: Vec<Tree>
}

#[derive(Debug)]
pub struct Let {
    pub id: Identifier,
    pub exp_type: Box<Tree>,
    pub exp: Box<Tree>
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
pub struct SumType {
    pub options: Vec<Tree>,
}

#[derive(Debug)]
pub struct ProductType {
    pub elements: Vec<Tree>,
}

#[derive(Debug)]
pub struct IdentifierType {
    pub name: Name
}

#[derive(Debug)]
pub struct FunctionType {
    pub from: Box<Tree>,
    pub to: Box<Tree>,
}

#[derive(Debug)]
pub struct ArrayType {
    pub value_type: Box<Tree>,
    pub length: usize,
}

#[derive(Debug)]
pub enum Tree {
    File(File),
    Let(Let),
    Identifier(Identifier),
    BinOp(BinOp),
    Lambda(Lambda),
    App(App),
    Number(Number),
    Boolean(Boolean),
    Text(Text),
    Tuple(Tuple),
    Array(Array),
    Block(Block),

    SumType(SumType),
    ProductType(ProductType),
    IdentifierType(IdentifierType),
    FunctionType(FunctionType),
    ArrayType(ArrayType),
    UnitType
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
