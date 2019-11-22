#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FunctionType {
    pub from: Box<Type>,
    pub to: Box<Type>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ProductType {
    pub elements: Vec<Type>
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ArrayType {
    pub value_type: Box<Type>,
    pub length: usize
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SumType {
    options: Vec<Type>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct AtomType {
    pub kind: AtomTypeKind
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AtomTypeKind {
    I8,
    I64,
    UI8,
    UI64,
    Boolean,
    Text
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExternType {
    pub name: String
}


#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    FunctionType(FunctionType),
    ProductType(ProductType),
    ArrayType(ArrayType),
    SumType(SumType),
    AtomType(AtomType),
    ExternType(ExternType),
    UnitType,
    UnknownType
}

impl Type {
    pub fn from_function(from: Type, to: Type) -> Type {
        Type::FunctionType(FunctionType { from: Box::from(from), to: Box::from(to) })
    }
    pub fn from_atom(kind: AtomTypeKind) -> Type {
        Type::AtomType(AtomType { kind: kind })
    }
    pub fn from_extern_string(name: String) -> Type {
        Type::ExternType(ExternType { name: name })
    }
    pub fn from_extern_str(name: &str) -> Type {
        Type::ExternType(ExternType { name: String::from(name) })
    }
    pub fn from_product(ts: Vec<Type>) -> Type {
        Type::ProductType(ProductType { elements: ts })
    }
    pub fn from_array_type(t: Type, l: usize) -> Type {
        Type::ArrayType(ArrayType { value_type: Box::from(t), length: l })
    }
}