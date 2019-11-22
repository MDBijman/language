use crate::data::hlr_ast::*;
use crate::data::hlr_ast;
use crate::data::types::*;
use crate::data::types;

use std::collections::HashMap;

pub struct Failure {
    pub message: String
}

impl Failure {
    fn new(m: &'static str) -> Failure {
        Failure { message: String::from(m) }
    }
    fn from(m: String) -> Failure {
        Failure { message: m }
    }
}

#[derive(Debug)]
struct Context {
    parent: Option<Box<Context>>,

    variables: HashMap<Name, Type>,
    types: HashMap<Name, Type>
}

impl Context {
    fn new() -> Context {
        Context { parent: None, variables: HashMap::new(), types: HashMap::new() }
    }

    fn add_variable(&mut self, name: Name, type_: Type) {
        self.variables.insert(name, type_);
    }

    fn get_variable(&mut self, name: &Name) -> Option<Type> {
        match self.variables.get(name) {
            None => None,
            Some(t) => Some(t.clone())
        }
    }

    fn add_type(&mut self, name: Name, type_: Type) {
        self.types.insert(name, type_);
    }

    fn get_type(&mut self, name: &Name) -> Option<Type> {
        match self.types.get(name) {
            None => None,
            Some(t) => Some(t.clone())
        }
    }

}

#[derive(Debug, Clone)]
struct Constraints {
    must_be: Option<Type>
}

impl Constraints {
    fn new() -> Constraints {
        Constraints { must_be: None }
    }

    fn must_be(t: Type) -> Constraints {
        Constraints { must_be: Some(t) }
    }
}

trait Checkable {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure>;
}

impl Checkable for File {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        for i in self.statements.iter() {
            i.check(ctx, None)?;
        }
        Ok(None)
    }
}

impl Checkable for Let {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match self.exp_type.check(ctx, None)? {
            Some(expected_type) => {
                ctx.add_variable(self.id.name.clone(), expected_type.clone());
                match self.exp.check(ctx, Some(Constraints::must_be(expected_type.clone())))? {
                    Some(actual_type) => {
                        if expected_type == actual_type {
                            Ok(None)
                        } else {
                            Err(Failure::from(format!("Let type {:?} does not match expression type {:?}", expected_type, actual_type)))
                        } 
                    },
                    None => Err(Failure::new("Expected let expression to yield type"))
                }
            }
            None => Err(Failure::new("Expected let rhs to return a type")),
        }
    }
}

impl Checkable for Block {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        let mut it = self.statements.iter().peekable();
        while let Some(stmt) = it.next() {
            if it.peek().is_some() {
                stmt.check(ctx, None)?;
            } else {
                return stmt.check(ctx, ctr);
            }
        };

        Err(Failure::new("Expected block.check to return type"))
    }
}

impl Checkable for Identifier {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match ctx.get_variable(&self.name) {
            None => Err(Failure::from(format!("Unknown variable {}", self.name.text))),
            Some(t) => {
                Ok(Some(t))
            }
        }
    }
}

impl Checkable for BinOp {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match self.op_type {
            BinOpType::Mult | BinOpType::Plus => {
                let lhs_type = self.lhs.check(ctx, ctr.clone())?.unwrap();
                let rhs_type = self.rhs.check(ctx, ctr)?.unwrap();

                if lhs_type == rhs_type {
                    Ok(Some(lhs_type))
                } else {
                    Err(Failure::new("BinOp Types must be equal"))
                }
            },
            BinOpType::ArrIndex => {
                match self.lhs.check(ctx, ctr.clone())?.unwrap() {
                    Type::ArrayType(types::ArrayType{ value_type: elem_type, length: _}) => {
                        let rhs_should_be = Type::AtomType(types::AtomType{ kind: AtomTypeKind::UI64 });
                        let rhs_ctr = Some(Constraints::must_be(rhs_should_be));
                        self.rhs.check(ctx, rhs_ctr)?;

                        Ok(Some(*elem_type))
                    },
                    _ => Err(Failure::new("Lhs of array index operation must have array type"))
                }
            }
        }
    }
}

impl Checkable for Lambda {
    fn check(&self, ctx: &mut Context, constraints: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match constraints {
            None => Err(Failure::new("Expected constraints")),
            Some(constraints) => {
                match constraints.must_be {
                    None => Err(Failure::new("Expected equality constraints")),
                    Some(type_to_be) => {
                        match type_to_be {
                            Type::FunctionType(types::FunctionType { from: from_type, to: to_type }) => {
                                match *from_type {
                                    Type::ProductType(types::ProductType { elements: elems }) => {
                                        if elems.len() != self.parameters.len() {
                                            return Err(Failure::new("Product type constraint not the same length as function parameter list"));
                                        }

                                        for (id, t) in self.parameters.iter().zip(elems.clone().into_iter()) {
                                            ctx.add_variable(id.name.clone(), t);
                                        }

                                        self.body.check(ctx, Some(Constraints::must_be(*to_type.clone())))?;
                                        Ok(Some(Type::from_function(Type::from_product(elems), *to_type)))
                                    },
                                    Type::UnitType => {
                                        self.body.check(ctx, Some(Constraints::must_be(*to_type.clone())))?;
                                        Ok(Some(Type::from_function(Type::UnitType, *to_type)))
                                    },
                                    t => {
                                        if self.parameters.len() != 1 {
                                            return Err(Failure::new("Lambda has multiple parameters but single type was given"));
                                        }

                                        ctx.add_variable(self.parameters.get(0).unwrap().name.clone(), t.clone());
                                        self.body.check(ctx, Some(Constraints::must_be(*to_type.clone())))?;
                                        Ok(Some(Type::from_function(t, *to_type)))
                                    }
                                }
                            },
                            _ => Err(Failure::new("Expected function constraint"))
                        }
                    }
                }
            }
        }
    }
}


impl Checkable for App {
    fn check(&self, ctx: &mut Context, constraints: Option<Constraints>) -> Result<Option<Type>, Failure> {
        let fn_type = self.fn_exp.check(ctx, None)?;

        match fn_type {
            Some(Type::FunctionType(ft)) => {
                let param_type = self.param_exp.check(ctx, None)?;
                match param_type {
                    Some(pt) => {
                        if *ft.from == pt {
                            Ok(Some(*ft.to))
                        } else {
                            Err(Failure::from(format!("Param type {:?} does not match argument type {:?}", pt, ft.from)))
                        }
                    }
                    _ => Err(Failure::new("Expected parameters to evaluate to type"))
                }
            },
            _ => Err(Failure::new("Expected function type result"))
        }
    }
}

impl Checkable for Number {
    fn check(&self, ctx: &mut Context, constraints: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match constraints {
            Some(c) => {
                match c.must_be {
                    Some(t) => {
                        match t {
                            Type::AtomType(AtomType{ kind: AtomTypeKind::I64 }) => 
                                Ok(Some(Type::AtomType(AtomType{ kind: AtomTypeKind::I64 }))),
                            Type::AtomType(AtomType{ kind: AtomTypeKind::I8 }) => 
                                Ok(Some(Type::AtomType(AtomType{ kind: AtomTypeKind::I8 }))),
                            Type::AtomType(AtomType{ kind: AtomTypeKind::UI64 }) => 
                                Ok(Some(Type::AtomType(AtomType{ kind: AtomTypeKind::UI64 }))),
                            Type::AtomType(AtomType{ kind: AtomTypeKind::UI8 }) => 
                                Ok(Some(Type::AtomType(AtomType{ kind: AtomTypeKind::UI8 }))),
                            _ => Err(Failure::new("Expected integer constraint"))
                        }
                    },
                    None => panic!("Expected a constraint")
                }
            },
            None => Ok(Some(Type::AtomType(AtomType{ kind: AtomTypeKind::I64 })))
        }
    }
}

impl Checkable for Boolean {
    fn check(&self, ctx: &mut Context, constraints: Option<Constraints>) -> Result<Option<Type>, Failure> {
        Ok(Some(Type::AtomType(AtomType{ kind: AtomTypeKind::Boolean })))
    }
}

impl Checkable for Text {
    fn check(&self, ctx: &mut Context, constraints: Option<Constraints>) -> Result<Option<Type>, Failure> {
        Ok(Some(Type::from_atom(AtomTypeKind::Text)))
    }
}

impl Checkable for Tuple {
    fn check(&self, ctx: &mut Context, constraints: Option<Constraints>) -> Result<Option<Type>, Failure> {
        let mut elems: Vec<Type> = Vec::new();

        for n in self.elements.iter() {
            match n.check(ctx, Some(Constraints::new()))? {
                Some(t) => elems.push(t),
                None => return Err(Failure::new("Expected type result")),
            }
        }

        Ok(Some(Type::ProductType(types::ProductType{ elements: elems })))
    }
}

impl Checkable for Array {
    fn check(&self, ctx: &mut Context, constraints: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match self.elements.len() {
            0 => match constraints {
                Some(c) => Ok(Some(c.must_be.unwrap())),
                None => Err(Failure::new("Empty array requires type constraint to determine array type"))
            },
            l => {
                let elem_constraints = match constraints {
                    Some(c) => match c.must_be.unwrap() {
                        Type::ArrayType(types::ArrayType{ value_type: vt, .. }) => Some(Constraints::must_be(*vt)),
                        _ => return Err(Failure::new("Cannot satisfy non-array type constraint when checking array"))
                    },
                    None => None
                };

                let mut it = self.elements.iter();

                let first = match it.next().unwrap().check(ctx, elem_constraints.clone())? {
                    None => return Err(Failure::new("Expected type results")),
                    Some(t) => t
                };

                for n in it {
                    match n.check(ctx, elem_constraints.clone())? {
                        Some(t) => if t != first {
                            return Err(Failure::new("Array values must be of equal type"))
                        },
                        None => return Err(Failure::new("Expected type result")),
                    }
                }

                Ok(Some(Type::ArrayType(types::ArrayType{ value_type: Box::from(first), length: l })))
            }
        }
    }
}

impl Checkable for hlr_ast::SumType {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        Ok(None)
    }
}

impl Checkable for hlr_ast::ProductType {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        let mut res: Vec<Type> = Vec::new();
        for elem in self.elements.iter() {
            match elem.check(ctx, None)? {
                None => return Err(Failure::new("Expected type result")),
                Some(t) => res.push(t)
            }
        }
        Ok(Some(types::Type::ProductType(types::ProductType { elements: res })))
    }
}

impl Checkable for IdentifierType {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match ctx.get_type(&self.name) {
            None => Err(Failure::from(format!("Unknown type identifier: {:}", self.name))),
            Some(t) => Ok(Some(t))
        }
    }
}

impl Checkable for hlr_ast::FunctionType {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match self.from.check(ctx, None)? {
            None => Err(Failure::new("Expected lhs to return type")),
            Some(t) => {
                match self.to.check(ctx, None)? {
                    None => Err(Failure::new("Expected rhs to return type")),
                    Some(t2) => {
                        Ok(Some(Type::from_function(t, t2)))
                    }
                }
            }
        }
    }
}

impl Checkable for hlr_ast::ArrayType {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match self.value_type.check(ctx, None)? {
            None => Err(Failure::new("Expected array value type to return type")),
            Some(t) => {
                Ok(Some(Type::from_array_type(t, self.length)))
            }
        }
    }
}

impl Checkable for Tree {
    fn check(&self, ctx: &mut Context, ctr: Option<Constraints>) -> Result<Option<Type>, Failure> {
        match self {
            Tree::File(n) => n.check(ctx, ctr),
            Tree::Let(n) => n.check(ctx, ctr),
            Tree::Block(n) => n.check(ctx, ctr),
            Tree::Identifier(n) => n.check(ctx, ctr),
            Tree::BinOp(n) => n.check(ctx, ctr),
            Tree::Lambda(n) => n.check(ctx, ctr),
            Tree::App(n) => n.check(ctx, ctr),
            Tree::Number(n) => n.check(ctx, ctr),
            Tree::Boolean(n) => n.check(ctx, ctr),
            Tree::Text(n) => n.check(ctx, ctr),
            Tree::Tuple(n) => n.check(ctx, ctr),
            Tree::Array(n) => n.check(ctx, ctr),
            Tree::SumType(n) => n.check(ctx, ctr),
            Tree::ProductType(n) => n.check(ctx, ctr),
            Tree::IdentifierType(n) => n.check(ctx, ctr),
            Tree::FunctionType(n) => n.check(ctx, ctr),
            Tree::ArrayType(n) => n.check(ctx, ctr),
            Tree::UnitType => Ok(Some(Type::UnitType))
        }
    }
}

pub fn check(t: &Tree) -> Result<Option<Type>, Failure> {
    let mut ctx = Context::new();

    ctx.add_variable(Name::from("std.print"), Type::from_function(Type::from_atom(AtomTypeKind::Text), types::Type::UnitType));
    ctx.add_variable(Name::from("std.println"), Type::from_function(Type::from_atom(AtomTypeKind::Text), types::Type::UnitType));
    ctx.add_variable(Name::from("std.read"), Type::from_function(Type::from_atom(AtomTypeKind::Text), Type::from_atom(AtomTypeKind::Text)));
    ctx.add_variable(Name::from("std.to_string"), Type::from_function(Type::from_atom(AtomTypeKind::UI8), Type::from_atom(AtomTypeKind::Text)));

    ctx.add_type(Name::from("ui8"), Type::from_atom(AtomTypeKind::UI8));
    ctx.add_type(Name::from("string"), Type::from_atom(AtomTypeKind::Text));

    let r = t.check(&mut ctx, None);
    r
}