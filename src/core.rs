use crate::types::{LangVal, FullEnv, Result};
use crate::eval::eval;

fn add(args: Vec<LangVal>, _: &mut FullEnv) -> Result<LangVal> {
    let mut res: f64 = 0.0;

    for arg in args {
        if let LangVal::Number(n) = arg {
            res += n;
        } else {
            Err("+ expected a number")?;
        }
    }

    Ok(LangVal::Number(res))
}

fn multiply(args: Vec<LangVal>, _: &mut FullEnv) -> Result<LangVal> {
    let mut res: f64 = 1.0;

    for arg in args {
        if let LangVal::Number(n) = arg {
            res *= n;
        } else {
            Err("* expected a number")?;
        }
    }

    Ok(LangVal::Number(res))
}

fn subtract(args: Vec<LangVal>, _: &mut FullEnv) -> Result<LangVal> {

    if args.len() == 0 {
        Err("- got too few arguments")?;
    }
    if args.len() == 1 {
        if let LangVal::Number(n) = args[0] {
            return Ok(LangVal::Number(-1.0 * n));
        } else {
            Err("- expected a number")?;
        }
    }

    let mut set: bool = false;
    let mut res: f64 = 1.0;

    for arg in args {
        if let LangVal::Number(n) = arg {
            if set {
                res -= n;
            } else {
                set = true;
                res = n;
            }
        } else {
            Err("+ expected a number")?;
        }
    }

    Ok(LangVal::Number(res))
}

fn divide(args: Vec<LangVal>, _: &mut FullEnv) -> Result<LangVal> {

    if args.len() == 0 {
        Err("/ got too few arguments")?;
    }
    if args.len() == 1 {
        if let LangVal::Number(n) = args[0] {
            if n == 0.0 {
                Err("Division by 0")?;
            } else {
                return Ok(LangVal::Number(1.0 / n));
            }
        } else {
            Err("/ expected a number")?;
        }
    }

    let mut set: bool = false;
    let mut res: f64 = 1.0;

    for arg in args {
        if let LangVal::Number(n) = arg {
            if set {
                if n == 0.0 {
                    Err("Division by 0")?;
                } else {
                    res /= n;
                }
            } else {
                set = true;
                res = n;
            }
        } else {
            Err("+ expected a number")?;
        }
    }

    Ok(LangVal::Number(res))
}

pub fn def_excl(args: Vec<LangVal>, env: &mut FullEnv) -> Result<LangVal> {
    if args.len() != 2 {
        Err(format!("def! expected 2 arguments, got {}", args.len()))?
    }

    match &args[0] {
       LangVal::Symbol(s) => {
           let val = eval(args[1].clone(), env)?;

           env.set(s.clone(), val.clone());

           Ok(val)
       }
       _ => Err("Cannot use def! on a non-symbol")?
    }
}

pub fn let_star(args: Vec<LangVal>, env: &mut FullEnv) -> Result<LangVal> {
    Err("let* not implemented")?
}

pub fn make_core_env() -> FullEnv {
    let mut ret = FullEnv::new();

    // arithmetic expressions
    ret.set("+".to_string(), LangVal::Function(add));
    ret.set("*".to_string(), LangVal::Function(multiply));
    ret.set("-".to_string(), LangVal::Function(subtract));
    ret.set("/".to_string(), LangVal::Function(divide));
    ret.set("def!".to_string(), LangVal::SpecialFunction(def_excl));
    ret.set("let*".to_string(), LangVal::SpecialFunction(let_star));

    ret
}