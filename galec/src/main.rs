mod tokenizer;
mod parser;
mod checker;
mod data;
mod interpreter;
mod lowerer;
mod graph_checker;

mod collections;
mod utils;
use std::fs;
fn main() {
    let contents = std::fs::read_to_string("simple.gale").expect("Error");
    let tokens = tokenizer::tokenize(&contents);
    let maybe_tree = parser::parse(&tokens).unwrap();
    let mlr_tree = lowerer::lower(&maybe_tree).unwrap();

    let g = graph_checker::gen_node_graph(&mlr_tree);
    let gs = utils::graphviz::print_as_graphviz(&g, 
        |n| { format!("{}", mlr_tree.get_node_value(n as usize).unwrap()) },
        |e| { String::from("") }
    );
    fs::write("./graphs/mlr.dot", gs).expect("Unable to write file");

    let (scope_graph, node_scopes) = graph_checker::gen_scope_graph(&mlr_tree);
    let scope_graph_dot = utils::graphviz::print_as_graphviz(&scope_graph, 
        |n| { format!("{}", node_scopes.get(n).unwrap()) },
        |e| { String::from("") }
    );
    fs::write("./graphs/mlr_scope_graph.dot", scope_graph_dot).expect("Unable to write file");

    let (g2, e2) = graph_checker::gen_type_dependencies(&g, &mlr_tree);
    let g2s = utils::graphviz::print_as_graphviz(&g2, 
        |n| { format!("{}", mlr_tree.get_node_value(n as usize).unwrap()) },
        |e| { match e2.get(e) {
            None => String::from(""),
            Some(d) => format!(" {}", d)
        } }
    );
    fs::write("./graphs/mlr_type.dot", g2s).expect("Unable to write file");

    /*
    let v2 = graph_checker::solve_type_dependencies(&g2, &e2);
    let g2s2 = utils::graphviz::print_as_graphviz(&g2, 
        |n| { match v2.get(n) {
            None => String::from(""),
            Some(d) => format!("{:?}", d)
        } },
        |e| { String::from("") }
    );
    fs::write("./graphs/mlr_types.dot", g2s2).expect("Unable to write file");
    */

    /*match maybe_tree {
        Ok(tree) => match checker::check(&tree) {
            Ok(_) => match lowerer::lower(&tree) {
                Ok(ltree) => match interpreter::interpret(ltree) {
                    Ok(v) => { 
                        println!("Exit value: {}", v)
                    },
                    Err(interpreter::Failure { message: t }) => println!("{:?}", t)
                },
                Err(lowerer::Failure { message: t }) => println!("{:?}", t)
            },
            Err(checker::Failure { message: t }) => println!("{:?}", t)
        },
        Err(err) => println!("{:?}", err)
    };*/
}
