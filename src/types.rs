use std::error::Error;
use std::rc::Rc;
use std::cell::RefCell;

pub type Result<T> = std::result::Result<T, Box<dyn Error>>;
pub type Hashmap = std::collections::HashMap<String, LangVal>;
pub type LangFunction = fn(Vec<LangVal>, Env) -> Result<LangVal>;
pub type TCOFunction = fn(Vec<LangVal>, Env) -> Result<(LangVal, Env)>;

#[derive(Clone)]
#[allow(dead_code)]
pub enum LangVal {
    // definitely gonna be used
    Nil,
    Boolean(bool),
    List(Vec<LangVal>),
    Vector(Vec<LangVal>),
    Number(f64),
    String(String),
    Symbol(String),
    Hashmap(Hashmap),
    Function(LangFunction),
    SpecialFunction(LangFunction), // functions where arguments are given in raw and unevaluated
    TCOFunction(TCOFunction), // TCO optimized function that needs to be directly implemented in the loop
    DefinedFunction {
        symbols: Vec<String>,
        ast: Box<LangVal>,
        env: Env,
        min_args: usize,
        is_variadic: bool
    },
    // quotes, etc
    WithSpecial((String, Rc<LangVal>))
}

#[allow(dead_code)]
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
    pub fn try_boolean(self) -> Option<bool> {
        if let LangVal::Boolean(v) = self { Some(v) } else { None }
    }
}

// environment implementation
pub struct EnvStruct {
    data: RefCell<std::collections::HashMap<String, LangVal>>,
    outer: Option<Env>
}
pub type Env = Rc<EnvStruct>;

pub fn env_push(outer: Option<Env>) -> Env {
    Rc::new(EnvStruct {
        data: RefCell::new(Default::default()),
        outer
    })
}

pub fn env_find(env: &Env, key: &String) -> Option<Env> {
    match (env.data.borrow().contains_key(key), env.outer.clone()) {
        (true, _) => Some(env.clone()),
        (false, Some(outer)) => env_find(&outer, key),
        _ => None
    }
}

pub fn env_get(env: &Env, key: &String) -> Result<LangVal> {
    let found = env_find(env, key);
    match found {
        Some(env) =>
            Ok(env.data
                .borrow()
                .get(key)
                .unwrap()
                .clone()),
        None => Err(format!("Symbol {} not found", key))?
    }
}

pub fn env_set(env: &Env, key: &str, val: LangVal) {
    env.data.borrow_mut().insert(key.to_string(), val);
}








