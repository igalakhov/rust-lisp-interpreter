use crate::types::{LangVal, FullEnv, Result};
use crate::eval::{eval, eval_ast};
use itertools::{Itertools, zip};
use std::rc::Rc;

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

fn fn_def(args: Vec<LangVal>, env: &mut FullEnv) -> Result<LangVal> {
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

fn fn_let(args: Vec<LangVal>, env: &mut FullEnv) -> Result<LangVal> {
    if args.len() != 2 {
        Err(format!("let* expected 2 arguments, got {}", args.len()))?
    }

    let mut binds: Vec<LangVal> = Default::default();

    match &args[0] {
        LangVal::List(v)|LangVal::Vector(v) => {
            binds = v.clone();
        }
        _ => {
            Err("First argument of let* must be list or vector")?;
        }
    };

    if binds.len() % 2 != 0 {
        Err("Second argument of let must have even parity")?;
    }

    env.push();

    for (k, v) in binds.into_iter().tuples() {
        fn_def(vec![k, v], env)?;
    }

    let ret = eval(args[1].clone(), env)?;

    env.pop();

    Ok(ret)
}

fn fn_do(args: Vec<LangVal>, env: &mut FullEnv) -> Result<LangVal> {
    let n = args.len();

    if n == 0 {
        Err("do expected at least 1 argument, got 0")?;
    }

    Ok(eval_ast(LangVal::List(args), env)?.try_list().unwrap()[n-1].clone())
}

fn fn_list(args: Vec<LangVal>, _: &mut FullEnv) -> Result<LangVal> {
    Ok(LangVal::List(args))
}

fn fn_list_q(args: Vec<LangVal>, _: &mut FullEnv) -> Result<LangVal> {
    if args.len() != 1 {
        Err(format!("list? expected 1 argument, got {}", args.len()))?;
    }
    match &args[0] {
        LangVal::List(_) => Ok(LangVal::Boolean(true)),
        _ => Ok(LangVal::Boolean(false))
    }
}

fn fn_empty_q(args: Vec<LangVal>, _: &mut FullEnv) -> Result<LangVal> {
    if args.len() != 1 {
        Err(format!("empty? expected 1 argument, got {}", args.len()))?;
    }
    match &args[0] {
        LangVal::List(v) => Ok(LangVal::Boolean(v.len() == 0)),
        _ => Err("empty? expected a list")?
    }
}

fn fn_count(args: Vec<LangVal>, _: &mut FullEnv) -> Result<LangVal> {
    if args.len() != 1 {
        Err(format!("count expected 1 argument, got {}", args.len()))?;
    }
    match &args[0] {
        LangVal::List(v) => Ok(LangVal::Number(v.len() as f64)),
        LangVal::Nil => Ok(LangVal::Number(0.0)),
        _ => Err("count expected a list")?
    }
}

fn fn_if(args: Vec<LangVal>, env: &mut FullEnv) -> Result<LangVal> {
    if args.len() < 2 {
        Err(format!("if expected at least 2 arguments, got {}", args.len()))?;
    }
    if args.len() > 3 {
        Err(format!("if expected at most 3 arguments, got {}", args.len()))?;
    }

    let true_case = args[1].clone();
    let false_case = if args.len() == 3 {args[2].clone()} else {LangVal::Nil};

    match eval(args[0].clone(), env)? {
        LangVal::Boolean(b) => {
            if b {
                Ok(eval(true_case, env)?)
            } else {
                Ok(eval(false_case, env)?)
            }
        }
        LangVal::Nil => Ok(eval(false_case, env)?),
        _ => Err("if expected a boolean as first argument")?
    }
}

fn fn_eq(args: Vec<LangVal>, env: &mut FullEnv) -> Result<LangVal> {
    if args.len() != 2 {
        Err(format!("= expected exactly 2 arguments, got {}", args.len()))?;
    }

    match (&args[0], &args[1]) {
        (LangVal::Number(a), LangVal::Number(b)) => Ok(LangVal::Boolean(a == b)),
        (LangVal::Boolean(a), LangVal::Boolean(b)) => Ok(LangVal::Boolean(a == b)),
        (LangVal ::String(a), LangVal::String(b))=> Ok(LangVal::Boolean(a == b)),
        (LangVal::Nil, LangVal::Nil) => Ok(LangVal::Boolean(true)),
        (_, _) => Ok(LangVal::Boolean(false))
    }
}

fn fn_fn(args: Vec<LangVal>, env: &mut FullEnv) -> Result<LangVal> {
    if args.len() != 2 {
        Err(format!("fn* expected exactly 2 arguments, got {}", args.len()))?;
    }

    if args[0].clone().try_list().is_none() {
        Err("fn* expected list of symbols as first argument")?;
    }

    let mut symbols = vec![];

    for i in args[0].clone().try_list().unwrap() {
        match i {
            LangVal::Symbol(s) => {
                symbols.push(s);
            }
            _ => Err("fn* expected list of symbols as first argument")?
        }
    }

    let ast = args[1].clone();

    Ok(LangVal::DefinedFunction {
        symbols,
        ast: Box::new(ast),
        env: env.clone()
    })
}

pub fn make_core_env() -> FullEnv {
    let mut ret = FullEnv::new();

    // normal functions
    ret.set("+".to_string(), LangVal::Function(add));
    ret.set("*".to_string(), LangVal::Function(multiply));
    ret.set("-".to_string(), LangVal::Function(subtract));
    ret.set("/".to_string(), LangVal::Function(divide));
    ret.set("list".to_string(), LangVal::Function(fn_list));
    ret.set("list?".to_string(), LangVal::Function(fn_list_q));
    ret.set("empty?".to_string(), LangVal::Function(fn_empty_q));
    ret.set("count".to_string(), LangVal::Function(fn_count));
    ret.set("=".to_string(), LangVal::Function(fn_eq));

    // special functions
    ret.set("def!".to_string(), LangVal::SpecialFunction(fn_def));
    ret.set("let*".to_string(), LangVal::SpecialFunction(fn_let));
    ret.set("do".to_string(), LangVal::SpecialFunction(fn_do));
    ret.set("if".to_string(), LangVal::SpecialFunction(fn_if));
    ret.set("fn*".to_string(), LangVal::SpecialFunction(fn_fn));

    ret
}