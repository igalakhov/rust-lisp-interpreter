use crate::types::{LangVal, FullEnv, Result, Hashmap};

pub fn eval(val: LangVal, env: &mut FullEnv) -> Result<LangVal> {
    match val {
        LangVal::List(to_eval) => {
            if to_eval.len() == 0 {
                Ok(LangVal::List(vec![]))
            } else {
                let func = eval_ast(to_eval[0].clone(), env)?;
                let args = to_eval[1..].to_vec();

                match func { // need to know if we should evaluate args or not
                    LangVal::Function(f) => {
                        let args = eval_ast(LangVal::List(args), env)?.try_list().unwrap();
                        f(args, env)
                    }
                    LangVal::SpecialFunction(f) => {
                        f(args, env)
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
