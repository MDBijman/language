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

enum EitherFunction {
    Function(Function),
    NativeFunction(NativeFunction)
}

#[derive(Debug)]
pub struct Program {
    ast: Tree,
    functions: HashMap<Name, Function>,
}

impl Program {
    fn new(ast: Tree) -> Program {
        Program { ast: ast, functions: HashMap::new() }
    }

    fn extend(&mut self, name: Name, func: Function) {
        self.functions.insert(name, func);
    }

    fn lookup(&self, name: &Name) -> Option<&Function> {
       self.functions.get(name)
    }

    fn get_name(&self, name: NodeId) -> Option<&Name> {
        match self.ast.get_node_value(name) {
            Some(Node::Identifier(id)) => Some(&id.name),
            _ => None
        }
    }

    fn get_node(&self, n: NodeId) -> Option<&Node> {
        self.ast.get_node_value(n)
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

fn add_function(program: &mut Program, env: &mut Environment, name: Name, f: Function) {
    env.extend(name.clone(), Value::from_function(name.text.clone()));
    program.extend(name, f);
}

impl Runnable for File {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        match program.lookup(&Name::from_str("main")) {
            Some(Function::GaleFunction(g)) => {
                for id in g.parameters.iter() {
                    let name = program.get_name(*id).unwrap();
                    env.extend(name.clone(), values::Value::from_num(3));
                }
                g.interpret(program, env)
            }
            None => Err(Failure::new("No main")),
            _ => unimplemented!()
        }
    }
}

impl Runnable for NodeId {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        program.get_node(*self).unwrap().interpret(program, env)
    }
}

impl Runnable for GaleFunction {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        self.implementation.interpret(program, env)
    }
}

impl Runnable for NativeFunction {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        Ok((self.implementation)(program, env))
    }
}

impl Runnable for Function  {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        match self {
            Function::GaleFunction(g) => g.interpret(program, env),
            Function::NativeFunction(f) => f.interpret(program, env),
        }
    }
}

impl Runnable for Let {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        let v = self.exp.interpret(program, env)?;

        env.extend(program.get_name(self.id).unwrap().clone(), v);

        Ok(values::Value::Void)
    }
}

impl Runnable for Seq {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        let mut it = self.elements.iter().peekable();
        while let Some(e) = it.next() {
            if it.peek().is_some() {
                (*e).interpret(program, env)?;
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
        let param_values = self.param.interpret(program, env)?;

        let function = program.lookup(program.get_name(self.fn_name).unwrap()).unwrap();

        let param_names = match function {
            Function::NativeFunction(f) => f.parameters.clone(),
            Function::GaleFunction(g) => g.parameters.iter().map(|x| program.get_name(*x).unwrap().clone()).collect()
        };

        let mut new_env = Environment::new();

        match param_values {
            Value::Tuple(values::Tuple{ elements }) => {
                // Even though this does exactly the same as the catchall branch of the match
                // it has to be duplicated because a match guard does not allow a move on elements
                if elements.len() == 1 {
                    new_env.extend(param_names[0].clone(), Value::from_tuple(elements));
                } else {
                    assert_eq!(param_names.len(), elements.len());
                    for (p, v) in param_names.into_iter().zip(elements.into_iter()) {
                        new_env.extend(p, v);
                    }
                }
            },
            v => {
                assert_eq!(param_names.len(), 1);
                new_env.extend(param_names[0].clone(), v);
            }
        };

        function.interpret(program, &mut new_env)
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

impl Runnable for Node {
    fn interpret(&self, program: &Program, env: &mut Environment) -> Result<values::Value, Failure> {
        match self {
            Node::Let(n) => n.interpret(program, env),
            Node::Seq(n) => n.interpret(program, env),
            Node::Identifier(n) => n.interpret(program, env),
            Node::BinOp(n) => n.interpret(program, env),
            Node::Number(n) => n.interpret(program, env),
            Node::Text(n) => n.interpret(program, env),
            Node::Apply(n) => n.interpret(program, env),
            Node::Tuple(n) => n.interpret(program, env),
            Node::Array(n) => n.interpret(program, env),
            Node::File(n) => n.interpret(program, env),
            _ => Err(Failure::new("Cannot interpret this node"))
        }
    }
}


pub fn interpret(t: Tree) -> Result<values::Value, Failure> {
    let mut p = Program::new(t);
    let mut env = Environment::new();

    add_function(&mut p, &mut env, Name::from_str("std.print"), Function::new_native(Name::from_str("std.print"), vec![Name::from_str("in")], print));
    add_function(&mut p, &mut env, Name::from_str("std.println"), Function::new_native(Name::from_str("std.println"), vec![Name::from_str("in")], println));
    add_function(&mut p, &mut env, Name::from_str("std.read"), Function::new_native(Name::from_str("std.read"), vec![Name::from_str("in")], read_file));
    add_function(&mut p, &mut env, Name::from_str("std.to_string"), Function::new_native(Name::from_str("std.to_string"), vec![Name::from_str("in")], to_string));

    // This is quite messy, since getting the root node requires an immutable borrow but adding the functions requires a mutable one, 
    // and then interpreting requires an immutable one again
    let mut program_funcs = Vec::new();

    {
        let root = p.get_node(root_id).unwrap();
        match root {
            Node::File(f) => {
                for func in f.functions.iter() {
                    match p.get_node(*func) {
                        Some(Node::Function(Function::GaleFunction(g))) => {
                            let f_name = p.get_name(g.name).unwrap();
                            env.extend(f_name.clone(), Value::from_function(f_name.text.clone()));
                            program_funcs.push((f_name.clone(), Function::GaleFunction(g.clone())))
                        },
                        _ => unimplemented!()
                    }
                };
            },
            _ => unimplemented!()
        };
    };

    for (n, f) in program_funcs.into_iter() {
        p.extend(n, f);
    }

    let root = p.get_node(root_id).unwrap();
    root.interpret(&p, &mut env)
}
