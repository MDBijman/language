#[derive(PartialEq, Debug)]
pub enum Token {
    Error,

    Quoted(String),
    Identifier(String),
    Number(i64),

    Type,
    True,
    False,
    Match,
    Module,
    Public,
    Ref,
    Import,
    If,
    While,
    Function,
    Let,
    Elseif,
    Else,

    RightArrow,
    FatRightArrow,
    LeftSquareBracket,
    RightSquareBracket,
    LeftRoundBracket,
    RightRoundBracket,
    LeftCurlyBracket,
    RightCurlyBracket,
    Backslash,
    Colon,
    Semicolon,
    Pipe,
    Comma,

    Equals,
    DoubleEquals,
    SmallerThan,
    GreaterThan,
    Modulo,
    Or,
    Plus,
    Mul,
    Not,
    ArrIndex
}

#[derive(PartialEq, Debug)]
enum SimpleToken {
    Error,
    Number(i64),
    Word(String),
    Punctuation(String),
    Quoted(String)
}

fn parse_word<T: Iterator<Item = char>>(it: &mut std::iter::Peekable<T>) -> String {
    let mut word = String::new();
    while let Some(&c) = it.peek()  {
        if c.is_alphanumeric() || c == '.' || c == '_' {
            it.next();
            word.push(c);
        } else {
            break;
        }
    }
    return word;
}

fn parse_number<T: Iterator<Item = char>>(it: &mut std::iter::Peekable<T>) -> i64 {
    let mut r: i64 = 0;
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                it.next();
                r = r * 10 + (c.to_digit(10).unwrap() as i64);
            }
            _ => break
        }
    } 
    return r;
}

fn parse_quoted<T: Iterator<Item = char>>(it: &mut std::iter::Peekable<T>) -> String {
    it.next();
    let mut word = String::new();
    while let Some(&c) = it.peek()  {
        if c == '\"' {
            it.next();
            break;
        } else {
            it.next();
            word.push(c);
        }
    }
    return word;
}

fn to_simple_tokens(code: &String) -> Vec<SimpleToken> {
    let mut result = Vec::new();
    let mut it = code.chars().peekable();
    while let Some(&c) = it.peek() {
        match c {
            '0'..='9' => {
                result.push(SimpleToken::Number(parse_number(&mut it)));
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                result.push(SimpleToken::Word(parse_word(&mut it)));
            }
            '\"' => {
                result.push(SimpleToken::Quoted(parse_quoted(&mut it)));
            }
            c if c.is_whitespace() => {
                it.next();
            }
            '[' | ']' | '(' | ')' | ':' | '\\' | '{' | '}' | ';' | '%' | '+' | '*' | '<' | ',' => {
                it.next();
                result.push(SimpleToken::Punctuation(c.to_string()));
            }
            '-' => {
                it.next();
                if let Some(&c) = it.peek() {
                    match c {
                        '>' => {
                            result.push(SimpleToken::Punctuation(String::from("->")));
                            it.next();
                        },
                        _ => result.push(SimpleToken::Punctuation('-'.to_string()))
                    }
                }
            }
            '=' => {
                it.next();
                if let Some(&c) = it.peek() {
                    match c {
                        '>' => {
                            result.push(SimpleToken::Punctuation(String::from("=>")));
                            it.next();
                        },
                        '=' => {
                            result.push(SimpleToken::Punctuation(String::from("==")));
                            it.next();
                        },
                        _ => result.push(SimpleToken::Punctuation('='.to_string()))
                    }
                }
            }
            '|' => {
                it.next();
                if let Some(&c) = it.peek() {
                    match c {
                        '|' => {
                            result.push(SimpleToken::Punctuation(String::from("||")));
                            it.next();
                        },
                        _ => result.push(SimpleToken::Punctuation('|'.to_string()))
                    }
                }
            },
            '!' => {
                it.next();
                if let Some(&c) = it.peek() {
                    match c {
                        '!' => {
                            result.push(SimpleToken::Punctuation(String::from("!!")));
                            it.next();
                        },
                        _ => {
                            result.push(SimpleToken::Punctuation(String::from("!")));
                        }
                    } 
                }
            },
            t => {
                it.next();
                println!("{:?}", t);
                result.push(SimpleToken::Error);
            }
        }
    }

    result
}

fn to_tokens(words: &Vec<SimpleToken>) -> Vec<Token> {
    words.iter()
        .map(|x| {
            match x {
                SimpleToken::Number(n) => Token::Number(*n),
                SimpleToken::Word(w) => match w.as_ref() {
                    "type" => Token::Type,
                    "true" => Token::True,
                    "false" => Token::False,
                    "match" => Token::Match,
                    "module" => Token::Module,
                    "public" => Token::Public,
                    "ref" => Token::Ref,
                    "import" => Token::Import,
                    "if" => Token::If,
                    "while" => Token::While,
                    "fn" => Token::Function,
                    "let" => Token::Let,
                    "elseif" => Token::Elseif,
                    "else" => Token::Else,
                    _ => Token::Identifier(w.to_string())
                }
                SimpleToken::Punctuation(p) => match p.as_ref() {
                    "->" => Token::RightArrow,
                    "=>" => Token::FatRightArrow,
                    "[" => Token::LeftSquareBracket,
                    "]" => Token::RightSquareBracket,
                    "(" => Token::LeftRoundBracket,
                    ")" => Token::RightRoundBracket,
                    "{" => Token::LeftCurlyBracket,
                    "}" => Token::RightCurlyBracket,
                    ":" => Token::Colon,
                    ";" => Token::Semicolon,
                    "," => Token::Comma,
                    "|" => Token::Pipe,
                    "==" => Token::DoubleEquals,
                    "=" => Token::Equals,
                    "\\" => Token::Backslash,
                    "<" => Token::SmallerThan,
                    ">" => Token::GreaterThan,
                    "%" => Token::Modulo,
                    "||" => Token::Or,
                    "+" => Token::Plus,
                    "*" => Token::Mul,
                    "!!" => Token::ArrIndex,
                    "!" => Token::Not,
                    _ => Token::Error
                }
                SimpleToken::Quoted(q) => {
                    Token::Quoted(q.to_string())
                }
                _ => Token::Error
            }
        })
        .collect()
}

pub fn tokenize(code: &std::string::String) -> Vec<Token> {
    let simple_tokens = to_simple_tokens(code);
    let tokenized = to_tokens(&simple_tokens);
    tokenized
}