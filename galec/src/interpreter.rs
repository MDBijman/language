use crate::data::mlr_ast::*;
use crate::data::values;
use crate::data::values::{Value};

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
pub struct Program {
    functions: HashMap<Name, Function>
}

impl Program {
    fn new() -> Program {
        Program { functions: HashMap::new() }
    }

    fn extend(&mut self, func: Function) {
        self.functions.insert(func.name.clone(), func);
    }

    fn lookup(&self, name: &Name) -> Option<&Function> {
        self.functions.get(&name)
    }
}

#[derive(Debug)]
pub struct Environment {
    variables: HashMap<Name, values::Value>,
}

impl Environment {
    fn new() -> Environment {
        Environment { variables: HashMap::new() }
    }

    fn extend(&mut self, name: Name, value: values::Value) {
        self.variables.insert(name, value);
    }

    fn lookup(&self, name: &Name) -> Option<&values::Value> {
        self.variables.get(&name)
    }
}

trait Runnable {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure>;
}

fn print(_program: &Program, env: &mut Environment) -> Value {
    match env.lookup(&Name::from_str("in")) {
        Some(v) => print!("{}", v),
        None => panic!()
    };
    Value::Void
}

fn println(_program: &Program, env: &mut Environment) -> Value {
    match env.lookup(&Name::from_str("in")) {
        Some(v) => println!("{}", v),
        None => panic!()
    };
    Value::Void
}

fn read_file(_program: &Program, env: &mut Environment) -> Value {
    match env.lookup(&Name::from_str("in")) {
        Some(Value::Text(t)) => {
            let text = std::fs::read_to_string(&t.text).unwrap();
            Value::from_text_string(text)
        }
        _ => panic!()
    }
}

fn to_string(_program: &Program, env: &mut Environment) -> Value {
    match env.lookup(&Name::from_str("in")) {
        Some(v) => Value::from_text_string(format!("{:?}", v)),
        _ => panic!()
    }
}

impl File {
    fn add_function(program: &mut Program, env: &mut Environment, f: Function) {
        env.extend(f.name.clone(), Value::from_function(f.name.text.clone()));
        program.extend(f);
    }

    fn interpret(&mut self) -> Result<values::Value, Failure> {
        let mut env = Environment::new();
        let mut program = Program::new();

        for f in self.functions.drain(0..) {
            env.extend(f.name.clone(), Value::from_function(f.name.text.clone()));
            program.extend(f);
        }

        File::add_function(&mut program, &mut env, Function::from_native(Name::from_str("std.print"), vec![Identifier::from_str("in")], print));
        File::add_function(&mut program, &mut env, Function::from_native(Name::from_str("std.println"), vec![Identifier::from_str("in")], println));
        File::add_function(&mut program, &mut env, Function::from_native(Name::from_str("std.read"), vec![Identifier::from_str("in")], read_file));
        File::add_function(&mut program, &mut env, Function::from_native(Name::from_str("std.to_string"), vec![Identifier::from_str("in")], to_string));

        match program.lookup(&Name::from_str("main")) {
            Some(f) => {
                for id in f.parameters.iter() {
                    env.extend(id.name.clone(), values::Value::from_num(3));
                }
                f.interpret(&program, &mut env)
            }
            None => Err(Failure::new("No main"))
        }
    }
}

impl Function {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        match &self.implementation {
            FunctionImpl::Code(t) => t.interpret(program, env),
            FunctionImpl::Native(func) => Ok(func(program, env))
        }
    }
}

impl Runnable for Let {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        let v = self.exp.interpret(program, env)?;
        env.extend(self.id.name.clone(), v);
        Ok(values::Value::Void)
    }
}

impl Runnable for Seq {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        let mut it = self.elements.iter().peekable();
        while let Some(e) = it.next() {
            if it.peek().is_some() {
                e.interpret(program, env)?;
            } else {
                return e.interpret(program, env);
            };
        };

        Err(Failure::new("Expected seq to produce value"))
    }
}

impl Runnable for Identifier {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        match env.lookup(&self.name) {
            Some(v) => Ok(v.clone()),
            None => Err(Failure::from(String::from("Could not find identifier ") + &self.name.text))
        }
    }
}

impl Runnable for BinOp {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        let lhs = self.lhs.interpret(program, env)?;
        let rhs = self.rhs.interpret(program, env)?;

        match (lhs, rhs, self.op_type) {
            (Value::Number(values::Number{ value: v1 }), Value::Number(values::Number{ value: v2 }), BinOpType::Mult) => Ok(Value::Number(values::Number::from(v1 * v2))),
            (Value::Number(values::Number{ value: v1 }), Value::Number(values::Number{ value: v2 }), BinOpType::Plus) => Ok(Value::Number(values::Number::from(v1 + v2))),
            (Value::Array(values::Array{ elements: elems }), Value::Number(values::Number{ value: v }), BinOpType::ArrIndex) => Ok(elems.get(v as usize).unwrap().clone()),
            (lhs, rhs, _) => Err(Failure::from(format!("Invalid binop nodes: {:?}, {:?}", lhs, rhs)))
        } 
    }
}

impl Runnable for Number {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
       Ok(values::Value::from_num(self.value))
    }
}

impl Runnable for Text {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
       Ok(values::Value::from_text_string(self.text.clone()))
    }
}

impl Runnable for Boolean {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
       Ok(values::Value::from_bool(self.value))
    }
}

impl Runnable for Apply {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        let param_val = self.param.interpret(program, env)?;

        match program.lookup(&self.fn_name.name) {
            None => Err(Failure::new("Could not find function")),
            Some(f) => { 
                let mut new_env = Environment::new();
                match param_val {
                    Value::Tuple(values::Tuple{ elements }) => {
                        if elements.len() == 1 {
                            // Even though this does exactly as the catchall branch of the match
                            // it has to be duplicated because a match guard does not allow a move on elements
                            new_env.extend(f.parameters.get(0).unwrap().name.clone(), Value::from_tuple(elements))
                        } else {
                            assert_eq!(f.parameters.len(), elements.len());
                            for (p, v) in f.parameters.iter().zip(elements.into_iter()) {
                                new_env.extend(p.name.clone(), v);
                            }
                        }
                    },
                    _ => {
                        assert_eq!(f.parameters.len(), 1);
                        new_env.extend(f.parameters.get(0).unwrap().name.clone(), param_val)
                    }
                };
                Ok(f.interpret(program, &mut new_env)?)
            }
        }
    }
}

impl Runnable for Tuple {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        let mut res: Vec<Value> = Vec::new();
        for c in self.elements.iter() {
            res.push(c.interpret(program, env)?);
        }
        Ok(values::Value::from_tuple(res))
    }
}

impl Runnable for Array {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        let mut res: Vec<Value> = Vec::new();
        for c in self.elements.iter() {
            res.push(c.interpret(program, env)?);
        }
        Ok(values::Value::from_array(res))
    }
}

impl Runnable for Tree {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        match self {
            Tree::Let(n) => n.interpret(program, env),
            Tree::Seq(n) => n.interpret(program, env),
            Tree::Identifier(n) => n.interpret(program, env),
            Tree::BinOp(n) => n.interpret(program, env),
            Tree::Number(n) => n.interpret(program, env),
            Tree::Text(n) => n.interpret(program, env),
            Tree::Apply(n) => n.interpret(program, env),
            Tree::Tuple(n) => n.interpret(program, env),
            Tree::Array(n) => n.interpret(program, env),
            _ => Err(Failure::new("Cannot interpret this node"))
        }
    }
}

pub fn interpret(t: &mut File) -> Result<values::Value, Failure> {
    t.interpret()
}
