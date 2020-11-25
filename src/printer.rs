use crate::types::LangVal;

pub fn print_val(val: &LangVal) {
    println!("{}", pr_str(val, true));
}

pub fn pr_str(val: &LangVal, readable: bool) -> String {

    let fmt = |vals: &Vec<LangVal>| -> String {
        vals.into_iter()
            .map(|x| {pr_str(x, readable)})
            .collect::<Vec<String>>()
            .join(" ")
    };

    match val {
        LangVal::Nil => {
            "nil".to_string()
        }
        LangVal::Boolean(b) => {
            if *b { "true".to_string() } else { "false".to_string() }
        }
        LangVal::List(vals) => {
            format!("({})", fmt(vals))
        }
        LangVal::Vector(vals) => {
            format!("[{}]", fmt(vals))
        }
        LangVal::Number(num) => {
            format!("{}", num)
        }
        LangVal::String(str) => {
            if str.starts_with("\u{29e}") {
                format!(":{}", &str[2..]) // "\u{29e}" takes up 2 chars, not 1
            } else if readable {
                format!("\"{}\"", escape_str(str))
            } else {
                str.clone()
            }
        }
        LangVal::Hashmap(mp) => {
            format!("{{{}}}", mp.into_iter().map(|(k, v)| {
                format!("{} {}",
                        pr_str(&LangVal::String(k.clone()), readable),
                        pr_str(v, readable))
            }).collect::<Vec<String>>().join(" "))
        }
        LangVal::Function(_)|
        LangVal::SpecialFunction(_)|
        LangVal::TCOFunction(_)|
        LangVal::DefinedFunction {symbols: _, ast: _, env: _, is_variadic: _, min_args: _} => {
            "<function>".to_string()
        }
        LangVal::WithSpecial((name, val)) => {
            format!("({} {})", name, pr_str(val, readable))
        }
        LangVal::Symbol(sym) => sym.to_string()
    }
}

fn escape_str(s: &str) -> String {
    s.chars()
        .map(|c| match c {
            '"' => "\\\"".to_string(),
            '\n' => "\\n".to_string(),
            '\\' => "\\\\".to_string(),
            _ => c.to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
}