use std::error::Error;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum LangVal {
    List(Vec<LangVal>),
    Number(f64),
    Symbol(String)
}

