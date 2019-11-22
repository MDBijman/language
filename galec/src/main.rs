mod tokenizer;
mod parser;
mod checker;
mod data;
mod interpreter;
mod lowerer;

fn main() {
    let contents = std::fs::read_to_string("simple.gale").expect("Error");
    let tokens = tokenizer::tokenize(&contents);
    let maybe_tree = parser::parse(&tokens);

    match maybe_tree {
        Ok(tree) => match checker::check(&tree) {
            Ok(_) => match lowerer::lower(&tree) {
                Ok(mut ltree) => match interpreter::interpret(&mut ltree) {
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
    };
}
