use std::error::Error;
use std::rc::Rc;
use std::collections::HashMap;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub enum LangVal {
    // definitely gonna be used
    List(Vec<LangVal>),
    Vector(Vec<LangVal>),
    Number(f64),
    String(String),
    Symbol(String),
    Hashmap(HashMap<String, LangVal>),

    // quotes, etc
    WithSpecial((String, Rc<LangVal>))
}

