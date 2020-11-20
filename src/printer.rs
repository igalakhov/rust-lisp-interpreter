use crate::types::LangVal;

pub fn print_val(val: &LangVal) {
    println!("{}", to_string(val));
}

pub fn to_string(val: &LangVal) -> String {

    let fmt = |vals: &Vec<LangVal>| -> String {
        vals.into_iter()
            .map(to_string)
            .collect::<Vec<String>>()
            .join(" ")
    };

    match val {
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
            format!("{}", str)
        }
        LangVal::Hashmap(mp) => {
            format!("{{{}}}", mp.into_iter().map(|(k, v)| {
                format!("{} {}", k, to_string(v))
            }).collect::<Vec<String>>().join(" "))
        }
        LangVal::WithSpecial((name, val)) => {
            format!("({} {})", name, to_string(val))
        }
        LangVal::Symbol(sym) => sym.to_string()
    }
}