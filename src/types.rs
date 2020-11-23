use std::error::Error;
use std::rc::Rc;
use std::borrow::{Borrow, BorrowMut};
use crate::types::LangVal::Function;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Hashmap = std::collections::HashMap<String, LangVal>;
pub type Env = std::collections::HashMap<String, LangVal>;
pub type LangFunction = fn(Vec<LangVal>, &mut FullEnv) -> Result<LangVal>;

#[derive(Clone)]
pub enum LangVal {
    // definitely gonna be used
    Nil,
    List(Vec<LangVal>),
    Vector(Vec<LangVal>),
    Number(f64),
    String(String),
    Symbol(String),
    Hashmap(Hashmap),
    Function(LangFunction),
    SpecialFunction(LangFunction), // functions where arguments are given in raw and unevaluated
    // quotes, etc
    WithSpecial((String, Rc<LangVal>))
}

impl LangVal {
    pub fn try_function(self) -> Option<LangFunction> {
        if let LangVal::Function(v) = self { Some(v) } else { None }
    }
    pub fn try_list(self) -> Option<Vec<LangVal>> {
        if let LangVal::List(v) = self { Some(v) } else { None }
    }
    pub fn try_symbol(self) -> Option<String> {
        if let LangVal::Symbol(v) = self { Some(v) } else { None }
    }
}

// slightly better implementation to ensure O(1) for most operations


#[derive(Clone, Default)]
pub struct FullEnv {
    nodes: Vec<Env>,
    size: usize
}

impl FullEnv {

    pub fn new() -> FullEnv {
        let mut ret = FullEnv {
            nodes: vec![Default::default()],
            size: 1
        };

        ret
    }

    pub fn set(&mut self, key: String, val: LangVal) {
        self.nodes[self.size-1].insert(key, val);
    }

    pub fn get(&self, val: &LangVal) -> Result<LangVal> {
        match val {
            LangVal::Symbol(s) => self.get_str(s),
            _ => Err("Cannot lookup non-symbol")?
        }
    }

    pub fn get_str(&self, val: &String) -> Result<LangVal> {
        let env = self.find_str(val)?;

        Ok(env.get(val).unwrap().clone())
    }
    pub fn find(&self, val: &LangVal) -> Result<&Env> {
        match val {
            LangVal::Symbol(s) => self.find_str(s),
            _ => Err("Cannot lookup non-symbol")?
        }
    }

    pub fn find_str(&self, val: &String) -> Result<&Env> {
        for i in (0..self.size).rev() {
            if self.nodes[i].contains_key(val) {
                return Ok(&self.nodes[i]);
            }
        }

        Err(format!("Symbol {} not found", val))?
    }
}





