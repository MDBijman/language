use crate::tokenizer::Token;
use crate::data::hlr_ast::*;

#[derive(Debug)]
pub struct Failure {
    message: String
}

impl Failure {
    fn new(m: &'static str) -> Failure {
        Failure { message: String::from(m) }
    }
    fn from(m: String) -> Failure {
        Failure { message: m }
    }
}

fn peek_token<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>, t: Token) -> bool {
    match it.peek() {
        Some(&t2) if t == *t2 => true,
        _ => false
    }
}

fn consume_token<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>, t: Token) -> Result<(), Failure> {
    match it.next() {
        Some(t2) if t == *t2 => Ok(()),
        Some(t2) => Err(Failure { message: format!("Expected {:?}, got {:?}", t, t2) }),
        None => Err(Failure { message: format!("Expected {:?}, got nothing", t) })
    }
}

//////////////////////////

fn parse_name_data<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Name, Failure> {
    match it.next() {
        Some(Token::Identifier(i)) => Ok(Name { text: String::from(i) }),
        Some(a) => Err(Failure{ message: format!("Expected identifier, got {:?}", a) }),
        _ => Err(Failure{ message: format!("Expected identifier, got nothing") })
    }
}

fn parse_quoted<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Text, Failure> {
    match it.next() {
        Some(Token::Quoted(i)) => Ok(Text { text: String::from(i) }),
        Some(a) => Err(Failure{ message: format!("Expected string, got {:?}", a) }),
        _ => Err(Failure{ message: format!("Expected string, got nothing") })
    }
}

//////////////////////////

fn parse_identifier<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Identifier, Failure> {
    let data = parse_name_data(it)?;
    Ok(Identifier { name: data })
}

fn parse_identifier_type<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<IdentifierType, Failure> {
    let data = parse_name_data(it)?;
    Ok(IdentifierType { name: data })
}

fn parse_product_type<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    match peek_token(it, Token::LeftRoundBracket) {
        true => {
            consume_token(it, Token::LeftRoundBracket)?;

            if peek_token(it, Token::RightRoundBracket) {
                consume_token(it, Token::RightRoundBracket)?;
                return Ok(Tree::ProductType(ProductType { elements: Vec::new() }))
            }

            let mut res: Vec<Tree> = Vec::new();
            let t = parse_identifier_type(it)?;

            res.push(Tree::IdentifierType(t));

            while !peek_token(it, Token::RightRoundBracket) {
                consume_token(it, Token::Comma)?;
                let t = parse_identifier_type(it)?;
                res.push(Tree::IdentifierType(t));
            }

            consume_token(it, Token::RightRoundBracket)?;

            match res.len() {
                1 => Ok(res.into_iter().next().unwrap()),
                _ => Ok(Tree::ProductType(ProductType { elements: res }))
            }

        },
        false => {
            let id = parse_identifier_type(it)?;
            Ok(Tree::IdentifierType(id))
        }
    }
}

fn parse_function_type<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    let lhs = parse_product_type(it)?;
    if peek_token(it, Token::RightArrow) {
        consume_token(it, Token::RightArrow)?;
        let rhs = parse_function_type(it)?;
        Ok(Tree::FunctionType(FunctionType { from: Box::from(lhs), to: Box::from(rhs) }))
    } else {
        Ok(lhs)
    }
}

fn parse_sum_type<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    let mut res: Vec<Tree> = Vec::new();

    let p = parse_function_type(it)?;
    res.push(p);

    while peek_token(it, Token::Pipe) {
        consume_token(it, Token::Pipe)?;
        let p = parse_function_type(it)?;
        res.push(p);
    }

    Ok(match res.len() {
        1 => res.into_iter().next().unwrap(),
        _ => Tree::SumType(SumType { options: res })
    })
}


fn parse_array_type<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    consume_token(it, Token::LeftSquareBracket)?;
    let id = Tree::IdentifierType(parse_identifier_type(it)?);
    consume_token(it, Token::Semicolon)?;
    match parse_number(it)? {
        Tree::Number(n) => {
            consume_token(it, Token::RightSquareBracket)?;
            Ok(Tree::ArrayType(ArrayType { value_type: Box::from(id), length: n.value as usize }))
        },
        _ => Err(Failure::new("Expected array type length to be number"))
    }
}

fn parse_type<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    match it.peek() {
        Some(Token::LeftSquareBracket) => parse_array_type(it),
        _ => parse_sum_type(it)
    }
}

//////////////////////////

fn parse_number<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    match it.next() {
        Some(Token::Number(i)) => Ok(Tree::Number(Number::from(*i))),
        _ => Err(Failure{ message: String::from("Expected number") })
    }
}

fn parse_bool<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    match it.next() {
        Some(Token::True) => Ok(Tree::Boolean(Boolean::from(true))),
        Some(Token::False) => Ok(Tree::Boolean(Boolean::from(false))),
        _ => Err(Failure{ message: String::from("Expected bool") })
    }
}

fn parse_array<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    consume_token(it, Token::LeftSquareBracket)?;

    if peek_token(it, Token::RightSquareBracket) {
        consume_token(it, Token::RightSquareBracket)?;
        return Ok(Tree::Array(Array::from(Vec::new()))) 
    }

    let mut res: Vec<Tree> = Vec::new();
    res.push(parse_expression(it)?);

    while !peek_token(it, Token::RightSquareBracket) {
        consume_token(it, Token::Comma)?;
        res.push(parse_expression(it)?);
    }

    consume_token(it, Token::RightSquareBracket)?;

    Ok(Tree::Array(Array::from(res)))
}

fn parse_tuple<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    consume_token(it, Token::LeftRoundBracket)?;

    if peek_token(it, Token::RightRoundBracket) {
        consume_token(it, Token::RightRoundBracket)?;
        return Ok(Tree::Tuple(Tuple::from(Vec::new()))) 
    }

    let mut res: Vec<Tree> = Vec::new();
    res.push(parse_expression(it)?);

    while !peek_token(it, Token::RightRoundBracket) {
        consume_token(it, Token::Comma)?;
        res.push(parse_expression(it)?);
    }

    consume_token(it, Token::RightRoundBracket)?;

    match res.len() {
        1 => Ok(res.into_iter().next().unwrap()),
        _ => Ok(Tree::Tuple(Tuple::from(res)))
    }
}

fn parse_block<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    consume_token(it, Token::LeftCurlyBracket)?;

    if peek_token(it, Token::RightCurlyBracket) {
        consume_token(it, Token::RightCurlyBracket)?;
        return Ok(Tree::Block(Block::from(Vec::new())))
    }

    let mut res: Vec<Tree> = Vec::new();
    res.push(parse_statement(it)?);

    while !peek_token(it, Token::RightCurlyBracket) {
        consume_token(it, Token::Semicolon)?;
        res.push(parse_statement(it)?);
    }

    consume_token(it, Token::RightCurlyBracket)?;

    match res.len() {
        1 => Ok(res.into_iter().next().unwrap()),
        _ => Ok(Tree::Block(Block::from(res)))
    }
}

fn parse_value<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    match it.peek() {
        Some(&Token::Identifier(_)) => Ok(Tree::Identifier(parse_identifier(it)?)),
        Some(&Token::Quoted(_)) => Ok(Tree::Text(parse_quoted(it)?)),
        Some(&Token::Number(_)) => parse_number(it),
        Some(&Token::LeftSquareBracket) => parse_array(it),
        Some(&Token::LeftRoundBracket) => parse_tuple(it),
        Some(&Token::LeftCurlyBracket) => parse_block(it),

        v => Err(Failure{ message: String::from(format!("Expected value, got {:?}", v)) })
    }
}

fn parse_lambda<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    if peek_token(it, Token::Backslash) {
        consume_token(it, Token::Backslash)?;

        let param_list = if peek_token(it, Token::LeftRoundBracket) {
            consume_token(it, Token::LeftRoundBracket)?;

            let mut res = Vec::new();

            // If not a () param list
            if !peek_token(it, Token::RightRoundBracket) {
                res.push(parse_identifier(it)?);

                while peek_token(it, Token::Comma) {
                    consume_token(it, Token::Comma)?;
                    res.push(parse_identifier(it)?);
                } 
            }

            consume_token(it, Token::RightRoundBracket)?;

            res
        } else {
            vec![parse_identifier(it)?]
        };

        consume_token(it, Token::FatRightArrow)?;
        let body = parse_expression(it)?;
        Ok(Tree::Lambda(Lambda { parameters: param_list, body: Box::from(body) }))
    } else {
        parse_value(it)
    }
}

fn parse_application<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    let first = parse_lambda(it)?;
    match it.peek() {
        Some(Token::LeftRoundBracket) | Some(Token::Number(_)) | Some(Token::Identifier(_)) | Some(Token::LeftSquareBracket) | Some(Token::Quoted(_)) => {
            let second = parse_lambda(it)?;
            return Ok(Tree::App(App{ fn_exp: Box::from(first), param_exp: Box::from(second) }))
        },
        _ => Ok(first)
    }
}

fn parse_operation<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    let lhs = parse_application(it)?;

    if peek_token(it, Token::Mul) {
        consume_token(it, Token::Mul)?;
        let rhs = parse_operation(it)?;
        Ok(Tree::BinOp(BinOp { lhs: Box::from(lhs), rhs: Box::from(rhs), op_type: BinOpType::Mult }))
    } else if peek_token(it, Token::Plus) {
        consume_token(it, Token::Plus)?;
        let rhs = parse_operation(it)?;
        Ok(Tree::BinOp(BinOp { lhs: Box::from(lhs), rhs: Box::from(rhs), op_type: BinOpType::Plus }))
    } else if peek_token(it, Token::ArrIndex) {
        consume_token(it, Token::ArrIndex)?;
        let rhs = parse_operation(it)?;
        Ok(Tree::BinOp(BinOp { lhs: Box::from(lhs), rhs: Box::from(rhs), op_type: BinOpType::ArrIndex }))
    }  else {
        Ok(lhs)
    }
}

fn parse_expression<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    parse_operation(it)
}

//////////////////////////

fn parse_let<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    consume_token(it, Token::Let)?;
    let i = parse_identifier(it)?;
    consume_token(it, Token::Colon)?;
    let t = parse_type(it)?;
    consume_token(it, Token::Equals)?;
    let b = parse_expression(it)?;
    Ok(Tree::Let(Let { id: i, exp_type: Box::from(t), exp: Box::from(b) }))
}

fn parse_statement<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    match it.peek() {
        Some(Token::Let) => parse_let(it),
        None => Err(Failure::new("Expected let or expression, got end of stream")),
        _ => parse_expression(it)
    }
}

fn parse_file<'a>(it: &mut std::iter::Peekable<impl Iterator<Item = &'a Token>>) -> Result<Tree, Failure> {
    let mut funcs = Vec::new();
    while it.peek().is_some() {
        funcs.push(parse_statement(it)?);
        consume_token(it, Token::Semicolon)?;
    }
    Ok(Tree::File(File { statements: funcs }))
}

pub fn parse(tokens: &Vec<Token>) -> Result<Tree, Failure> {
    let mut i = tokens.iter().peekable();
    parse_file(&mut i)
}