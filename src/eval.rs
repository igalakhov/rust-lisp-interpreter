use crate::types::{LangVal, Result, Hashmap, Env, env_push, env_set, env_get};
use itertools::zip;

pub fn eval(start_val: LangVal, start_env: Env) -> Result<LangVal> {

    let mut val = start_val;
    let mut env = start_env;

    loop { // loop for TCO
        match val {
            LangVal::List(to_eval) => {
                if to_eval.len() == 0 {
                    return Ok(LangVal::List(vec![]));
                } else {
                    let func = eval(to_eval[0].clone(), env.clone())?;
                    let args = to_eval[1..].to_vec();

                    match func { // need to know if we should evaluate args or not
                        LangVal::Function(f) => {
                            let args = eval_ast(LangVal::List(args), env.clone())?.try_list().unwrap();
                            return f(args, env.clone());
                        }
                        LangVal::SpecialFunction(f) => {
                            return f(args, env.clone());
                        }
                        LangVal::DefinedFunction {
                            symbols,
                            ast,
                            env: other_env,
                            min_args,
                            is_variadic,
                        } => {
                            let args = eval_ast(LangVal::List(args),
                                env.clone())?.try_list().unwrap();

                            let (new_ast, new_env) = eval_defined(args, symbols, ast,
                                                                  min_args, is_variadic,
                                                                  &mut other_env.clone())?;

                            val = new_ast;
                            env = new_env;

                            continue;
                        }
                        LangVal::TCOFunction(f) => {
                            let (new_ast, new_env) = f(args, env.clone())?;

                            val = new_ast;
                            env = new_env;

                            continue; // loop instead of recursing, which lets us save some space
                        }
                        _ => {
                            Err("Expected function as first argument")?;
                        }
                    }
                }
            }
            _ => {
                return eval_ast(val, env);
            }
        }

        break; // we will never actually get here
    }

    panic!("Something went... really wrong")
}

pub fn eval_ast(val: LangVal, env: Env) -> Result<LangVal> {
    match val {
        LangVal::Symbol(s) => {
            env_get(&env, &s)
        }
        LangVal::List(elems) => {
            let mut ret = vec![];

            for i in elems {
                ret.push(eval(i, env.clone())?);
            }

            Ok(LangVal::List(ret))
        }
        LangVal::Vector(elems) => {
            let mut ret = vec![];

            for i in elems {
                ret.push(eval(i, env.clone())?);
            }

            Ok(LangVal::Vector(ret))
        }
        LangVal::Hashmap(mp) => {
            let mut ret = Hashmap::default();

            for (k, val) in mp {
                ret.insert(k, eval(val, env.clone())?);
            }

            Ok(LangVal::Hashmap(ret))
        }
        _ => Ok(val)
    }
}

fn eval_defined(args: Vec<LangVal>, symbols: Vec<String>, ast: Box<LangVal>,
                min_args: usize, is_variadic: bool, env: &Env)
-> Result<(LangVal, Env)> {
    if is_variadic {
        if args.len() < min_args {
            Err(format!("function expected at least {} arguments, got {}", min_args, args.len()))?;
        }
    } else {
        if args.len() != min_args {
            Err(format!("function expected {} arguments, got {}", min_args, args.len()))?;
        }
    }

    let env = env_push(Some(env.clone()));

    if is_variadic {
        for i in 0..(min_args) {
            env_set(&env, &symbols[i], args[i].clone());
        }
        env_set(&env, &symbols[min_args], LangVal::List(args[(min_args)..].to_vec()));
    } else {
        for (i, j) in zip(symbols, args) {
            env_set(&env, &i, j);
        }
    }

    Ok((*(ast.clone()), env))
}
