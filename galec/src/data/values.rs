use std::fmt;

#[derive(Debug, Clone)]
pub struct Number {
    pub value: i64
}

#[derive(Debug, Clone)]
pub struct Boolean {
    pub value: bool
}

#[derive(Debug, Clone)]
pub struct Tuple {
    pub elements: Vec<Value>
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String
}

#[derive(Debug, Clone)]
pub struct Text {
    pub text: String
}

#[derive(Debug, Clone)]
pub struct Array {
    pub elements: Vec<Value>
}

#[derive(Debug, Clone)]
pub struct ExternValue {
    pub values: Vec<Value>
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(Number),
    Boolean(Boolean),
    Tuple(Tuple),
    Function(Function),
    Text(Text),
    Array(Array),
    ExternValue(ExternValue),
    Void
}

impl Value {
    pub fn from_num(v: i64) -> Value {
        Value::Number(Number::from(v))
    }
    pub fn from_bool(v: bool) -> Value {
        Value::Boolean(Boolean { value: v })
    }
    pub fn from_tuple(v: Vec<Value>) -> Value {
        Value::Tuple(Tuple { elements: v })
    }
    pub fn from_function(n: String) -> Value {
        Value::Function(Function { name: n })
    }
    pub fn from_text(n: &str) -> Value {
        Value::Text(Text{ text: String::from(n) })
    }
    pub fn from_text_string(n: String) -> Value {
        Value::Text(Text{ text: n })
    }
    pub fn from_array(v: Vec<Value>) -> Value {
        Value::Array(Array { elements: v })
    }
    pub fn from_extern(vs: Vec<Value>) -> Value {
        Value::ExternValue(ExternValue { values: vs })
    }
}

impl Number {
    pub fn from(v: i64) -> Number {
        Number { value: v }
    }
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for elem in self.elements.iter() {
            write!(f, "{}", elem)?;
        }
        write!(f, ")")
    }
}
impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for elem in self.elements.iter() {
            write!(f, "{}", elem)?;
        }
        write!(f, "]")
    }
}
impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl fmt::Display for Text {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}
impl fmt::Display for ExternValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for v in self.values.iter() {
            write!(f, "{}", v)?;
        };
        Ok(())
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::Boolean(n) => write!(f, "{}", n),
            Value::Tuple(n) => write!(f, "{}", n),
            Value::Function(n) => write!(f, "{}", n),
            Value::Text(n) => write!(f, "{}", n),
            Value::Array(n) => write!(f, "{}", n),
            Value::ExternValue(n) => write!(f, "{}", n),
            Value::Void => write!(f, "void")
        }
    }
}