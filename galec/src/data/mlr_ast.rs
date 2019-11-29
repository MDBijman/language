use crate::interpreter::{ Program, Environment };
use crate::data::values::{ Value };
use crate::collections::flat_tree::{ FlatTree };
pub use crate::collections::flat_tree::{ NodeId, root_id, error_id };
//use std::collections::VecDeque;

// Medium Level Representation AST

pub type Tree = FlatTree<Node>;

// Function and File cannot appear everywhere

#[derive(Debug, Clone)]
pub struct GaleFunction {
    pub name: NodeId,
    pub parameters: Vec<NodeId>,
    pub implementation: NodeId
}

pub struct NativeFunction {
    pub name: Name,
    pub parameters: Vec<Name>,
    pub implementation: fn(&Program, &mut Environment) -> Value
}

#[derive(Debug)]
pub enum Function {
    GaleFunction(GaleFunction),
    NativeFunction(NativeFunction)
}

#[derive(Debug)]
pub struct File {
    pub functions: Vec<NodeId>
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
    pub id: NodeId,
    pub exp_type: NodeId,
    pub exp: NodeId
}

#[derive(Debug)]
pub struct Seq {
    pub elements: Vec<NodeId>
}

#[derive(Debug)]
pub struct Apply {
    pub fn_name: NodeId,
    pub param: NodeId
}

#[derive(Debug)]
pub struct BinOp {
    pub lhs: NodeId,
    pub rhs: NodeId,
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
    pub elements: Vec<NodeId>
}

#[derive(Debug)]
pub struct Array {
    pub elements: Vec<NodeId>
}

#[derive(Debug)]
pub struct SumType {
    pub options: Vec<NodeId>,
}

#[derive(Debug)]
pub struct ProductType {
    pub elements: Vec<NodeId>,
}

#[derive(Debug)]
pub struct IdentifierType {
    pub name: Name
}

#[derive(Debug)]
pub struct FunctionType {
    pub from: NodeId,
    pub to: NodeId,
}

#[derive(Debug)]
pub struct ArrayType {
    pub value_type: NodeId,
    pub length: usize,
}

#[derive(Debug)]
pub enum Node {
    File(File),
    Let(Let),
    Seq(Seq),
    Identifier(Identifier),
    BinOp(BinOp),
    Number(Number),
    Boolean(Boolean),
    Text(Text),
    Tuple(Tuple),
    Array(Array),
    Function(Function),
    Apply(Apply),

    SumType(SumType),
    ProductType(ProductType),
    IdentifierType(IdentifierType),
    FunctionType(FunctionType),
    ArrayType(ArrayType),
    UnitType
}

impl Node {
    pub fn from_text(s: String) -> Node {
        Node::Text(Text { text: s })
    }
    pub fn from_array(elems: Vec<NodeId>) -> Node {
        Node::Array(Array { elements: elems })
    }
    pub fn from_tuple(elems: Vec<NodeId>) -> Node {
        Node::Tuple(Tuple { elements: elems })
    }
    pub fn from_let(id: NodeId, exp_type: NodeId, exp: NodeId) -> Node {
        Node::Let(Let { id: id, exp_type: exp_type, exp: exp })
    }
    pub fn from_gale_fn(n: NodeId, p: Vec<NodeId>, b: NodeId) -> Node {
        Node::Function(Function::new_gale(n, p, b))
    }
    pub fn from_native_fn(n: Name, p: Vec<Name>, b: fn(&Program, &mut Environment) -> Value) -> Node {
        Node::Function(Function::new_native(n, p, b))
    }
    pub fn from_seq(ns: Vec<NodeId>) -> Node {
        Node::Seq(Seq::new(ns))
    }
    pub fn from_id(n: Name) -> Node {
        Node::Identifier(Identifier::from_name(n))
    }
    pub fn from_binop(l: NodeId, r: NodeId, op: BinOpType) -> Node {
        Node::BinOp(BinOp::from(l, r, op))
    }
    pub fn from_apply(id: NodeId, params: NodeId) -> Node {
        Node::Apply(Apply::new(id, params))
    }
    pub fn from_file(fns: Vec<NodeId>) -> Node {
        Node::File(File::from(fns))
    }
    pub fn from_num(n: i64) -> Node {
        Node::Number(Number::from(n))
    }
    pub fn from_bool(b: bool) -> Node {
        Node::Boolean(Boolean::from(b))
    }
    pub fn from_sum_type(elems: Vec<NodeId>) -> Node {
        Node::SumType(SumType { options: elems })
    }
    pub fn from_product_type(elems: Vec<NodeId>) -> Node {
        Node::ProductType(ProductType { elements: elems })
    }
    pub fn from_type_id(name: Name) -> Node {
        Node::IdentifierType(IdentifierType { name: name })
    }
    pub fn from_function_type(from: NodeId, to: NodeId) -> Node {
        Node::FunctionType(FunctionType { from: from, to: to })
    }
    pub fn from_array_type(elem_type: NodeId, len: usize) -> Node {
        Node::ArrayType(ArrayType { value_type: elem_type, length: len })
    }
    pub fn from_unit_type() -> Node {
        Node::UnitType
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Let(n) => write!(f, "Let"),
            Node::Seq(n) => write!(f, "Seq"),
            Node::Identifier(n) => write!(f, "Id({})", n.name),
            Node::BinOp(n) => write!(f, "{}", n.op_type),
            Node::Number(n) => write!(f, "{}", n.value),
            Node::Text(n) => write!(f, "{}", n.text),
            Node::Apply(n) => write!(f, "Call"),
            Node::Tuple(n) => write!(f, "Tuple"),
            Node::Array(n) => write!(f, "Array"),
            Node::File(n) => write!(f, "File"),
            Node::Function(n) => write!(f, "Fn"),
            Node::SumType(n) => write!(f, "SumType"),
            Node::ProductType(n) => write!(f, "ProductType"),
            Node::IdentifierType(n) => write!(f, "TypeId({})", n.name),
            Node::FunctionType(n) => write!(f, "FnType"),
            Node::ArrayType(n) => write!(f, "ArrayType"),
            Node::UnitType => write!(f, "UnitType"),
            n => unimplemented!() 
        }
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
    pub fn new() -> Name {
        Name::from_string(String::new())
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Function {
    pub fn new_gale(n: NodeId, p: Vec<NodeId>, b: NodeId) -> Function {
        Function::GaleFunction(GaleFunction { name: n, parameters: p, implementation: b })
    }
    pub fn new_native(n: Name, p: Vec<Name>, b: fn(&Program, &mut Environment) -> Value) -> Function {
        Function::NativeFunction(NativeFunction { name: n, parameters: p, implementation: b })
    }
}

impl GaleFunction {
    pub fn new(n: NodeId, p: Vec<NodeId>, b: NodeId) -> GaleFunction {
        GaleFunction { name: n, parameters: p, implementation: b }
    }
}

impl NativeFunction {
    pub fn new(n: Name, p: Vec<Name>, b: fn(&Program, &mut Environment) -> Value) -> NativeFunction {
        NativeFunction { name: n, parameters: p, implementation: b }
    }
}

impl std::fmt::Debug for NativeFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Native Function {{ name: {:?}, parameters: {:?} }}", self.name, self.parameters)
    }
}

impl File {
    pub fn new() -> File {
        File{ functions: Vec::new() }
    }
    pub fn from(fns: Vec<NodeId>) -> File {
        File{ functions: fns }
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
    pub fn new(id: NodeId, exp_type: NodeId, exp: NodeId) -> Let {
        Let { id: id, exp_type: exp_type, exp: exp }
    }
}

impl Seq {
    pub fn new(stmts: Vec<NodeId>) -> Seq {
        Seq { elements: stmts }
    }
}

impl Apply {
    pub fn new(id: NodeId, param: NodeId) -> Apply {
        Apply { fn_name: id, param: param }
    }
}

impl BinOp {
    pub fn from(lhs: NodeId, rhs: NodeId, op_type: BinOpType) -> BinOp {
        BinOp { lhs: lhs, rhs: rhs, op_type: op_type }
    }
}

impl std::fmt::Display for BinOpType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BinOpType::Plus => write!(f, "+"),
            BinOpType::Mult => write!(f, "*"),
            BinOpType::ArrIndex => write!(f, "!!"),
        }
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
    pub fn from(elems: Vec<NodeId>) -> Tuple {
        Tuple { elements: elems }
    }
}
