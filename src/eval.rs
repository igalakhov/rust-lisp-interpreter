use crate::types::{LangVal, FullEnv, Result, Hashmap};
use itertools::zip;
use std::rc::Rc;
use std::borrow::Borrow;

pub fn eval(val: LangVal, env: &mut FullEnv) -> Result<LangVal> {
    match val {
        LangVal::List(to_eval) => {
            if to_eval.len() == 0 {
                Ok(LangVal::List(vec![]))
            } else {
                let func = eval(to_eval[0].clone(), env)?;
                let args = to_eval[1..].to_vec();

                match func { // need to know if we should evaluate args or not
                    LangVal::Function(f) => {
                        let args = eval_ast(LangVal::List(args), env)?.try_list().unwrap();
                        f(args, env)
                    }
                    LangVal::SpecialFunction(f) => {
                        f(args, env)
                    }
                    LangVal::DefinedFunction {
                        symbols,
                        ast,
                        env: other_env
                    } => {
                        let args = eval_ast(LangVal::List(args), env)?.try_list().unwrap();
                        eval_defined(args, symbols, ast, &mut other_env.clone())
                    }
                    _ => {
                        Err("Expected function as first argument")?
                    }
                }

            }
        }
        _ => eval_ast(val, env)
    }
}

pub fn eval_ast(val: LangVal, env: &mut FullEnv) -> Result<LangVal> {
    match val {
        LangVal::Symbol(s) => {
            env.get_str(&s)
        }
        LangVal::List(elems) => {
            let mut ret = vec![];

            for i in elems {
                ret.push(eval(i, env)?);
            }

            Ok(LangVal::List(ret))
        }
        LangVal::Vector(elems) => {
            let mut ret = vec![];

            for i in elems {
                ret.push(eval(i, env)?);
            }

            Ok(LangVal::Vector(ret))
        }
        LangVal::Hashmap(mp) => {
            let mut ret = Hashmap::default();

            for (k, val) in mp {
                ret.insert(k, eval(val, env)?);
            }

            Ok(LangVal::Hashmap(ret))
        }
        _ => Ok(val)
    }
}

fn eval_defined(args: Vec<LangVal>, symbols: Vec<String>, ast: Box<LangVal>, env: &mut FullEnv)
-> Result<LangVal> {

    if args.len() != symbols.len() {
        Err("function got wrong number of arguments")?;
    }

    env.push();

    for (i, j) in zip(symbols, args) {
        env.set(i, j);
    }

    let ret = eval(*(ast.clone()), env);

    env.pop();

    ret
}
