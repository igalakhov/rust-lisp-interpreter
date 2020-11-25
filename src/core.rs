use crate::types::{LangVal, Result, Env, env_push, env_set};
use crate::eval::{eval, eval_ast};
use crate::reader;
use itertools::{Itertools, zip};
use crate::printer::pr_str;

fn add(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
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

fn multiply(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
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

fn subtract(args: Vec<LangVal>, _: Env) -> Result<LangVal> {

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

fn divide(args: Vec<LangVal>, _: Env) -> Result<LangVal> {

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

fn fn_def(args: Vec<LangVal>, env: Env) -> Result<LangVal> {
    if args.len() != 2 {
        Err(format!("def! expected 2 arguments, got {}", args.len()))?
    }

    match &args[0] {
       LangVal::Symbol(s) => {
           let val = eval(args[1].clone(), env.clone())?;

           env_set(&env, s, val.clone());

           Ok(val)
       }
       _ => Err("Cannot use def! on a non-symbol")?
    }
}

fn fn_let(args: Vec<LangVal>, env: Env) -> Result<(LangVal, Env)> {
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

    let env = env_push(Some(env));

    for (k, v) in binds.into_iter().tuples() {
        fn_def(vec![k, v], env.clone())?;
    }

    Ok((args[1].clone(), env))
}

fn fn_do(args: Vec<LangVal>, env: Env) -> Result<(LangVal, Env)> {
    let n = args.len();

    if n == 0 {
        Err("do expected at least 1 argument, got 0")?;
    }

    // evaluate the first n-1 arguments
    eval_ast(LangVal::List(args[..(args.len()-1)].to_vec()), env.clone())?;


    Ok((args[args.len()-1].clone(), env))
}

fn fn_list(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    Ok(LangVal::List(args))
}

fn fn_list_q(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    if args.len() != 1 {
        Err(format!("list? expected 1 argument, got {}", args.len()))?;
    }
    match &args[0] {
        LangVal::List(_) => Ok(LangVal::Boolean(true)),
        _ => Ok(LangVal::Boolean(false))
    }
}

fn fn_empty_q(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    if args.len() != 1 {
        Err(format!("empty? expected 1 argument, got {}", args.len()))?;
    }
    match &args[0] {
        LangVal::List(v)|
        LangVal::Vector(v) => Ok(LangVal::Boolean(v.len() == 0)),
        _ => Err("empty? expected a list")?
    }
}

fn fn_count(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    if args.len() != 1 {
        Err(format!("count expected 1 argument, got {}", args.len()))?;
    }
    match &args[0] {
        LangVal::List(v)|
        LangVal::Vector(v) => Ok(LangVal::Number(v.len() as f64)),
        LangVal::Nil => Ok(LangVal::Number(0.0)),
        _ => Err("count expected a list")?
    }
}

fn fn_if(args: Vec<LangVal>, env: Env) -> Result<(LangVal, Env)> {
    if args.len() < 2 {
        Err(format!("if expected at least 2 arguments, got {}", args.len()))?;
    }
    if args.len() > 3 {
        Err(format!("if expected at most 3 arguments, got {}", args.len()))?;
    }

    let true_case = args[1].clone();
    let false_case = if args.len() == 3 {args[2].clone()} else {LangVal::Nil};

    match eval(args[0].clone(), env.clone())? {
        LangVal::Boolean(b) => {
            if b {
                Ok((true_case, env))
            } else {
                Ok((false_case, env))
            }
        },
        LangVal::Number(_) => {
            Ok((true_case, env))
        }
        LangVal::List(_)|LangVal::Vector(_) => {
            Ok((true_case, env))
        }
        LangVal::String(_) => {
            Ok((true_case, env))
        }
        LangVal::Nil => Ok((false_case, env)),
        _ => Err("if expected a boolean as first argument")?
    }
}

fn fn_eq(args: Vec<LangVal>, env: Env) -> Result<LangVal> {
    if args.len() != 2 {
        Err(format!("= expected exactly 2 arguments, got {}", args.len()))?;
    }

    match (&args[0], &args[1]) {
        (LangVal::Number(a), LangVal::Number(b)) => Ok(LangVal::Boolean(a == b)),
        (LangVal::Boolean(a), LangVal::Boolean(b)) => Ok(LangVal::Boolean(a == b)),
        (LangVal ::String(a), LangVal::String(b))=> Ok(LangVal::Boolean(a == b)),
        (LangVal::Nil, LangVal::Nil) => Ok(LangVal::Boolean(true)),
        (LangVal::List(v1), LangVal::List(v2))|
        (LangVal::Vector(v1), LangVal::Vector(v2))|
        (LangVal::Vector(v1), LangVal::List(v2))|
        (LangVal::List(v1), LangVal::Vector(v2)) => {
            if v1.len() != v2.len() {
                return Ok(LangVal::Boolean(false))
            }
            for (i, j) in zip(v1, v2) {
                if !fn_eq(vec![i.clone(), j.clone()], env.clone())?.try_boolean().unwrap() {
                    return Ok(LangVal::Boolean(false));
                }
            }
            Ok(LangVal::Boolean(true))
        }
        (_, _) => Ok(LangVal::Boolean(false))
    }
}

fn fn_greater(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    if args.len() != 2 {
        Err(format!("> expected exactly 2 arguments, got {}", args.len()))?;
    }

    match (&args[0], &args[1]) {
        (LangVal::Number(a), LangVal::Number(b)) => Ok(LangVal::Boolean(a > b)),
        (_, _) => Err("Cannot compare given values")?
    }
}

fn fn_fn(args: Vec<LangVal>, env: Env) -> Result<LangVal> {
    if args.len() != 2 {
        Err(format!("fn* expected exactly 2 arguments, got {}", args.len()))?;
    }

    let mut symbols = vec![];

    match &args[0] {
        LangVal::List(v)|LangVal::Vector(v) => {
            for i in v {
                match i {
                    LangVal::Symbol(s) => {
                        symbols.push(s.clone());
                    }
                    _ => Err("fn* expected list of symbols as first argument")?
                }
            }
        }
        _ => Err("fn* expected list of symbols as first argument")?
    }

    let mut is_variadic = false;
    let mut min_args = symbols.len();
    let mut new_symbols = vec![];

    // check variadic
    for i in 0..symbols.len() {
        if symbols[i] == "&" {
            if is_variadic {
                Err("& only allowed to be used once in function signature")?;
            }

            is_variadic = true;
            min_args = i;
        } else {
            new_symbols.push(symbols[i].clone());
        }
    }

    if is_variadic {
        symbols = new_symbols;
    }

    let ast = args[1].clone();

    Ok(LangVal::DefinedFunction {
        symbols,
        ast: Box::new(ast),
        env: env.clone(),
        min_args,
        is_variadic,
    })
}

fn fn_pr_str(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    let s = args.into_iter().map(|x| {
        pr_str(&x, true)
    }).join(" ");

    Ok(LangVal::String(s))
}

fn fn_str(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    let s = args.into_iter().map(|x| {
        pr_str(&x, false)
    }).join("");

    Ok(LangVal::String(s))
}

fn fn_prn(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    let s = args.into_iter().map(|x| {
        pr_str(&x, true)
    }).join(" ");

    println!("{}", s);

    Ok(LangVal::Nil)
}

fn fn_println(args: Vec<LangVal>, _: Env) -> Result<LangVal> {
    let s = args.into_iter().map(|x| {
        pr_str(&x, false)
    }).join(" ");

    println!("{}", s);

    Ok(LangVal::Nil)
}

pub fn make_core_env() -> Env {
    let ret = env_push(None);

    // normal functions
    env_set(&ret, "+", LangVal::Function(add));
    env_set(&ret, "*", LangVal::Function(multiply));
    env_set(&ret, "-", LangVal::Function(subtract));
    env_set(&ret, "/", LangVal::Function(divide));
    env_set(&ret, "list", LangVal::Function(fn_list));
    env_set(&ret, "list?", LangVal::Function(fn_list_q));
    env_set(&ret, "empty?", LangVal::Function(fn_empty_q));
    env_set(&ret, "count", LangVal::Function(fn_count));
    env_set(&ret, "=", LangVal::Function(fn_eq));
    env_set(&ret, ">", LangVal::Function(fn_greater));
    env_set(&ret, "pr-str", LangVal::Function(fn_pr_str));
    env_set(&ret, "str", LangVal::Function(fn_str));
    env_set(&ret, "prn", LangVal::Function(fn_prn));
    env_set(&ret, "println", LangVal::Function(fn_println));

    // special functions
    env_set(&ret, "def!", LangVal::SpecialFunction(fn_def));
    env_set(&ret, "let*", LangVal::TCOFunction(fn_let));
    env_set(&ret, "do", LangVal::TCOFunction(fn_do));
    env_set(&ret, "if", LangVal::TCOFunction(fn_if));
    env_set(&ret, "fn*", LangVal::SpecialFunction(fn_fn));

    // functions defined using the language itself
    let defns = vec![
        // boolean functions
        "(def! not (fn* (a) (if a false true)))",
        "(def! or (fn* (a b) (if a true (if b true false))))",
        "(def! and (fn* (a b) (if a (if b true false) false)))",
        // comparisons
        "(def! >= (fn* (a b) (or (= a b) (> a b))))",
        "(def! < (fn* (a b) (not (>= a b))))",
        "(def! <= (fn* (a b) (not (> a b))))",
    ];

    for def in defns {
        eval(reader::read_str(def).unwrap(), ret.clone()).unwrap();
    }

    ret
}