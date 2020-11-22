use std::error::Error;
use std::rc::Rc;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Hashmap = std::collections::HashMap<String, LangVal>;

#[derive(Debug, Clone)]
pub enum LangVal {
    // definitely gonna be used
    List(Vec<LangVal>),
    Vector(Vec<LangVal>),
    Number(f64),
    String(String),
    Symbol(String),
    Hashmap(Hashmap),
    Function(fn(Vec<LangVal>) -> Result<LangVal>),

    // quotes, etc
    WithSpecial((String, Rc<LangVal>))
}

impl LangVal {
    pub fn as_function(self) -> fn(Vec<LangVal>) -> Result<LangVal> {
        if let LangVal::Function(f) = self {
            return f;
        };
        panic!("not a function");
    }

    pub fn as_list(self) -> Vec<LangVal> {
        if let LangVal::List(l) = self {
            return l;
        };
        panic!("not a function");
    }
}

pub type Env = HashMap<String, LangVal>;



