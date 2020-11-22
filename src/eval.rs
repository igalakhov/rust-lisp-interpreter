use crate::types::{LangVal, Env, Result, Hashmap};

pub fn eval(val: LangVal, env: &Env) -> Result<LangVal> {
    match val {
        LangVal::List(args) => {
            if args.len() == 0 {
                Ok(LangVal::List(vec![]))
            } else {
                let to_eval = eval_ast(LangVal::List(args), &env)?.as_list();
                let f = to_eval[0].clone().as_function();
                let args = to_eval[1..].to_vec();

                Ok(f(args)?)
            }
        }
        _ => eval_ast(val, env)
    }
}

pub fn eval_ast(val: LangVal, env: &Env) -> Result<LangVal> {
    match val {
        LangVal::Symbol(s) => {
            if env.contains_key(s.as_str()) {
                Ok(env.get(s.as_str()).cloned().unwrap())
            } else {
                Err(format!("symbol {} not defined", s))?
            }
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
