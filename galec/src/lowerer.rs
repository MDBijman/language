use crate::data::hlr_ast;
use crate::data::mlr_ast;

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

trait Lowerable {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure>;
}

impl Lowerable for hlr_ast::File {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        let mut f = mlr_ast::File::new();
        for statement in self.statements.iter() {
            match statement.lower()? {
                Some(mlr_ast::Tree::Function(func)) => { f.functions.push(func) },
                Some(_) => { return Err(Failure::new("Expected each statement to lower to function or nothing")); },
                None    => {},
            }
        }

        Ok(Some(mlr_ast::Tree::File(f)))
    }
}

impl Lowerable for hlr_ast::Let {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        match self.id.lower()? {
            Some(mlr_ast::Tree::Identifier(i)) => {
                match &*self.exp {
                    // Let of lambda lowers to function
                    hlr_ast::Tree::Lambda(l) => {
                        match l.body.lower()? {
                            Some(t) => {
                                let mut params = Vec::<mlr_ast::Identifier>::new();
                                for old_param in l.parameters.iter() {
                                    match old_param.lower()? {
                                        Some(mlr_ast::Tree::Identifier(id)) => params.push(id),
                                        _ => return Err(Failure::new("expected identifier"))
                                    }
                                }
                                Ok(Some(mlr_ast::Tree::Function(mlr_ast::Function::from_code(i.name, params, Box::from(t)))))
                            },
                            None => Err(Failure::new("Expected body to lower to tree"))
                        }
                    },
                    // Other lets
                    _ => match self.exp.lower()? {
                        Some(t) => {
                            Ok(Some(mlr_ast::Tree::Let(mlr_ast::Let::new(i, t))))
                        },
                        None => Err(Failure::new("Expected identifier to lower to tree"))
                    }
                }
            },
            Some(_) => Err(Failure::new("Expected identifier to lower to identifier")),
            None => Err(Failure::new("Expected identifier to lower to identifier, got none"))
        }
    }
}

impl Lowerable for hlr_ast::Block {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        let mut res: Vec<mlr_ast::Tree> = Vec::new();
        for s in self.statements.iter() {
            match s.lower()? {
                None => return Err(Failure::new("Expected tree result")),
                Some(t) => res.push(t)
            };
        }

        Ok(Some(mlr_ast::Tree::Seq(mlr_ast::Seq::new(res))))
    }
}

impl Lowerable for hlr_ast::Identifier {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        Ok(Some(mlr_ast::Tree::Identifier(mlr_ast::Identifier::from_string(self.name.text.clone()))))
    }
}

fn lower_op_type(op: hlr_ast::BinOpType) -> mlr_ast::BinOpType {
    match op {
        hlr_ast::BinOpType::Mult => mlr_ast::BinOpType::Mult,
        hlr_ast::BinOpType::Plus => mlr_ast::BinOpType::Plus,
        hlr_ast::BinOpType::ArrIndex => mlr_ast::BinOpType::ArrIndex,
    }
}

impl Lowerable for hlr_ast::BinOp {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        let lhs = self.lhs.lower()?;
        let rhs = self.rhs.lower()?;
        if lhs.is_none() || rhs.is_none() {
            return Err(Failure::new("Expected children to lower to tree"));
        };
        Ok(Some(mlr_ast::Tree::BinOp(mlr_ast::BinOp::from(lhs.unwrap(), rhs.unwrap(), lower_op_type(self.op_type)))))
    }
}

impl Lowerable for hlr_ast::Lambda {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        unimplemented!();
    }
}

impl Lowerable for hlr_ast::App {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        match &*self.fn_exp {
            hlr_ast::Tree::Identifier(id) => {
                match id.lower()? {
                    Some(mlr_ast::Tree::Identifier(id)) => match self.param_exp.lower()? {
                        Some(param) => Ok(Some(mlr_ast::Tree::Apply(mlr_ast::Apply::new(id, param)))),
                        _ => Err(Failure::new("Expected param to lower to tree}"))
                    }
                    _ => Err(Failure::new("Expected id to lower to id"))
                }
            },
            _ => unimplemented!()
        }
    }
}

impl Lowerable for hlr_ast::Number {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        Ok(Some(mlr_ast::Tree::Number(mlr_ast::Number::from(self.value))))
    }
}

impl Lowerable for hlr_ast::Boolean {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        Ok(Some(mlr_ast::Tree::Boolean(mlr_ast::Boolean::from(self.value))))
    }
}

impl Lowerable for hlr_ast::Text {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        Ok(Some(mlr_ast::Tree::from_text(self.text.clone())))
    }
}

impl Lowerable for hlr_ast::Tuple {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        let mut elems: Vec<mlr_ast::Tree> = Vec::new();
        for elem in self.elements.iter() {
            match elem.lower()? {
                None => return Err(Failure::new("Expected tree result")),
                Some(t) => elems.push(t)
            }
        }
        Ok(Some(mlr_ast::Tree::Tuple(mlr_ast::Tuple::from(elems))))
    }
}

impl Lowerable for hlr_ast::Array {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        let mut elems: Vec<mlr_ast::Tree> = Vec::new();
        for elem in self.elements.iter() {
            match elem.lower()? {
                None => return Err(Failure::new("Expected tree result")),
                Some(t) => elems.push(t)
            }
        }
        Ok(Some(mlr_ast::Tree::from_array(elems)))
    }
}

impl Lowerable for hlr_ast::Tree {
    fn lower(&self) -> Result<Option<mlr_ast::Tree>, Failure> {
        match self {
            hlr_ast::Tree::File(f) => f.lower(),
            hlr_ast::Tree::Let(f) => f.lower(),
            hlr_ast::Tree::Block(f) => f.lower(),
            hlr_ast::Tree::Identifier(f) => f.lower(),
            hlr_ast::Tree::BinOp(f) => f.lower(),
            hlr_ast::Tree::Lambda(f) => f.lower(),
            hlr_ast::Tree::App(f) => f.lower(),
            hlr_ast::Tree::Number(f) => f.lower(),
            hlr_ast::Tree::Boolean(f) => f.lower(),
            hlr_ast::Tree::Text(f) => f.lower(),
            hlr_ast::Tree::Tuple(f) => f.lower(),
            hlr_ast::Tree::Array(f) => f.lower(),
            hlr_ast::Tree::SumType(_) => Ok(None),
            hlr_ast::Tree::ProductType(_) => Ok(None),
            hlr_ast::Tree::IdentifierType(_) => Ok(None),
            hlr_ast::Tree::FunctionType(_) => Ok(None),
            hlr_ast::Tree::ArrayType(_) => Ok(None),
            hlr_ast::Tree::UnitType => Ok(None)
        }
    }
}


pub fn lower(t: &hlr_ast::Tree) -> Result<mlr_ast::File, Failure> {
    match t.lower() {
        Ok(Some(mlr_ast::Tree::File(f))) => Ok(f),
        _ => Err(Failure::new("Expected file"))
    }
}