use crate::data::hlr_ast;
use crate::data::mlr_ast;
use crate::data::mlr_ast::{ Tree, NodeId, root_id, error_id };

#[derive(Debug)]
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

fn map_to_value(id: Option<NodeId>, t: &Tree) -> Option<&mlr_ast::Node> {
    match id {
        Some(real_id) => {
            match t.get_node_value(real_id) {
                Some(n) => Some(n),
                None => None
            }
        }
        None => None
    }
}

fn map_to_id_value(id: Option<NodeId>, t: &Tree) -> Option<(NodeId, &mlr_ast::Node)> {
    match id {
        Some(real_id) => {
            match t.get_node_value(real_id) {
                Some(n) => Some((real_id, n)),
                None => None
            }
        }
        None => None
    }
}

trait Lowerable {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure>;
}

impl Lowerable for hlr_ast::File {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let file_id = t.new_node(mlr_ast::Node::File(mlr_ast::File::new()), p);

        let mut functions = Vec::new();
        for statement in self.statements.iter() {
            match statement.lower(t, file_id)? {
                Some(id) => functions.push(id),
                _ => ()
            }
        }

        t.set_node_value(file_id, mlr_ast::Node::from_file(functions));

        Ok(Some(file_id))
    }
}

impl Lowerable for hlr_ast::Let {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let let_id = t.new_node(mlr_ast::Node::from_let(error_id, error_id, error_id), p);

        let (id_id, id_val) = map_to_id_value(self.id.lower(t, let_id)?, t).unwrap();
        let name = match id_val {
            mlr_ast::Node::Identifier(i) => i.name.clone(),
            _ => return Err(Failure::new("Id must lower to id"))
        };

        let exp_type_id = self.exp_type.lower(t, let_id)?.unwrap();

        match &*self.exp {
            // Let of lambda lowers to function
            // So we overwrite the let node in the tree
            hlr_ast::Tree::Lambda(l) => {
                // Function body
                let (exp_id, exp_val) = map_to_id_value(l.body.lower(t, let_id)?, t).unwrap();

                // Function parameters
                let mut params = Vec::new();
                for old_param in l.parameters.iter() {
                    params.push(old_param.lower(t, let_id)?.unwrap());
                }
                
                t.set_node_value(let_id, mlr_ast::Node::from_gale_fn(id_id, params, exp_id));
                Ok(Some(let_id))
            },
            // Other lets
            _ => {
                let (exp_id, exp_val) = map_to_id_value(self.exp.lower(t, let_id)?, t).unwrap();
                t.set_node_value(let_id, mlr_ast::Node::from_let(id_id, exp_type_id, exp_id));
                Ok(Some(let_id))
            }
        }
    }
}

impl Lowerable for hlr_ast::Block {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let seq_id = t.new_node(mlr_ast::Node::from_seq(Vec::new()), p);

        let mut res: Vec<NodeId> = Vec::new();
        for s in self.statements.iter() {
            let s_id = s.lower(t, seq_id)?.unwrap();
            res.push(s_id);
        }

        t.set_node_value(seq_id, mlr_ast::Node::from_seq(res));

        Ok(Some(seq_id))
    }
}

impl Lowerable for hlr_ast::Identifier {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let id = t.new_node(mlr_ast::Node::from_id(mlr_ast::Name::from_string(self.name.text.clone())), p);
        Ok(Some(id))
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
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let binop_id = t.new_node(mlr_ast::Node::from_binop(error_id, error_id, lower_op_type(self.op_type)), p);
        let lhs = self.lhs.lower(t, binop_id)?;
        let rhs = self.rhs.lower(t, binop_id)?;
        if lhs.is_none() || rhs.is_none() {
            return Err(Failure::new("Expected children to lower to tree"));
        };

        match t.get_mut_node_value(binop_id).unwrap() {
            mlr_ast::Node::BinOp(b) => {
                b.lhs = lhs.unwrap();
                b.rhs = rhs.unwrap();
            },
            _ => panic!("Binop node was unexpectedly of different type")
        };

        Ok(Some(binop_id))
    }
}

impl Lowerable for hlr_ast::Lambda {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        unimplemented!();
    }
}

impl Lowerable for hlr_ast::App {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        match &*self.fn_exp {
            hlr_ast::Tree::Identifier(id) => {
                let call_id = t.new_node(mlr_ast::Node::from_apply(error_id, error_id), p);
                let (id_id, id_val) = map_to_id_value(id.lower(t, call_id)?, t).unwrap();
                let (param_id, param_val) = map_to_id_value(self.param_exp.lower(t, call_id)?, t).unwrap();

                t.set_node_value(call_id, mlr_ast::Node::from_apply(id_id, param_id));

                Ok(Some(call_id))
            },
            _ => unimplemented!()
        }
    }
}

impl Lowerable for hlr_ast::Number {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let num_id = t.new_node(mlr_ast::Node::from_num(self.value), p);
        Ok(Some(num_id))
    }
}

impl Lowerable for hlr_ast::Boolean {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let bool_id = t.new_node(mlr_ast::Node::from_bool(self.value), p);
        Ok(Some(bool_id))
    }
}

impl Lowerable for hlr_ast::Text {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let text_id = t.new_node(mlr_ast::Node::from_text(self.text.clone()), p);
        Ok(Some(text_id))
    }
}

impl Lowerable for hlr_ast::Tuple {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let tuple_id = t.new_node(mlr_ast::Node::from_tuple(Vec::new()), p);

        let mut elems: Vec<NodeId> = Vec::new();
        for elem in self.elements.iter() {
            elems.push(elem.lower(t, tuple_id)?.unwrap());
        }

        t.set_node_value(tuple_id, mlr_ast::Node::from_tuple(elems));

        Ok(Some(tuple_id))
    }
}

fn lower_multiple(ns: &Vec<hlr_ast::Tree>, t: &mut Tree, p: NodeId) -> Result<Vec<NodeId>, Failure> {
    let mut elems: Vec<NodeId> = Vec::new();
    for elem in ns.iter() {
        elems.push(elem.lower(t, p)?.unwrap());
    };
    Ok(elems)
}

impl Lowerable for hlr_ast::Array {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let array_id = t.new_node(mlr_ast::Node::from_array(Vec::new()), p);
        let elems = lower_multiple(&self.elements, t, array_id)?;
        t.set_node_value(array_id, mlr_ast::Node::from_array(elems));

        Ok(Some(array_id))
    }
}

impl Lowerable for hlr_ast::SumType {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let sum_id = t.new_node(mlr_ast::Node::from_sum_type(Vec::new()), p);
        let elems = lower_multiple(&self.options, t, sum_id)?;
        t.set_node_value(sum_id, mlr_ast::Node::from_sum_type(elems));
        Ok(Some(sum_id))
    }
}

impl Lowerable for hlr_ast::ProductType {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let prod_id = t.new_node(mlr_ast::Node::from_product_type(Vec::new()), p);
        let elems = lower_multiple(&self.elements, t, prod_id)?;
        t.set_node_value(prod_id, mlr_ast::Node::from_sum_type(elems));
        Ok(Some(prod_id))
    }
}

impl Lowerable for hlr_ast::IdentifierType {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        Ok(Some(t.new_node(mlr_ast::Node::from_type_id(mlr_ast::Name::from_string(self.name.text.clone())), p)))
    }
}

impl Lowerable for hlr_ast::FunctionType {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let t_id = t.new_node(mlr_ast::Node::from_function_type(error_id, error_id), p);
        let from_id = self.from.lower(t, t_id)?.unwrap();
        let to_id = self.to.lower(t, t_id)?.unwrap();
        t.set_node_value(t_id, mlr_ast::Node::from_function_type(from_id, to_id));
        Ok(Some(t_id))
    }
}

impl Lowerable for hlr_ast::ArrayType {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        let t_id = t.new_node(mlr_ast::Node::from_array_type(error_id, 0), p);
        let vt_id = self.value_type.lower(t, t_id)?.unwrap();
        t.set_node_value(t_id, mlr_ast::Node::from_array_type(vt_id, self.length));
        Ok(Some(t_id))
    }
}

fn make_unit_type(t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
    Ok(Some(t.new_node(mlr_ast::Node::from_unit_type(), p)))
}

impl Lowerable for hlr_ast::Tree {
    fn lower(&self, t: &mut Tree, p: NodeId) -> Result<Option<NodeId>, Failure> {
        match self {
            hlr_ast::Tree::File(f) => f.lower(t, p),
            hlr_ast::Tree::Let(f) => f.lower(t, p),
            hlr_ast::Tree::Block(f) => f.lower(t, p),
            hlr_ast::Tree::Identifier(f) => f.lower(t, p),
            hlr_ast::Tree::BinOp(f) => f.lower(t, p),
            hlr_ast::Tree::Lambda(f) => f.lower(t, p),
            hlr_ast::Tree::App(f) => f.lower(t, p),
            hlr_ast::Tree::Number(f) => f.lower(t, p),
            hlr_ast::Tree::Boolean(f) => f.lower(t, p),
            hlr_ast::Tree::Text(f) => f.lower(t, p),
            hlr_ast::Tree::Tuple(f) => f.lower(t, p),
            hlr_ast::Tree::Array(f) => f.lower(t, p),
            hlr_ast::Tree::SumType(f) => f.lower(t, p),
            hlr_ast::Tree::ProductType(f) => f.lower(t, p),
            hlr_ast::Tree::IdentifierType(f) => f.lower(t, p),
            hlr_ast::Tree::FunctionType(f) => f.lower(t, p),
            hlr_ast::Tree::ArrayType(f) => f.lower(t, p),
            hlr_ast::Tree::UnitType => make_unit_type(t, p),
        }
    }
}


pub fn lower(t: &hlr_ast::Tree) -> Result<Tree, Failure> {
    let mut nt = Tree::new_empty();

    t.lower(&mut nt, root_id)?.unwrap();

    Ok(nt)
}