use crate::types::LangVal;

pub fn print_val(val: &LangVal) {
    println!("{}", to_string(val));
}

pub fn to_string(val: &LangVal) -> String {
    match val {
        LangVal::List(vals) => {
            format!("({})",
                    vals.into_iter()
                        .map(to_string)
                        .collect::<Vec<String>>()
                        .join(" "))
        }
        LangVal::Number(num) => {
            format!("{}", num)
        }
        LangVal::Symbol(sym) => sym.to_string()
    }
}