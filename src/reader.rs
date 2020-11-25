use regex::Regex;
use crate::types::{Result, LangVal};
use std::rc::Rc;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Reader {
    tokens: Vec<String>,
    pos: usize
}

impl Reader {
    fn next(&mut self) -> Result<String> {
        self.pos += 1;
        Ok(self.tokens
            .get(self.pos-1)
            .ok_or("Not enough tokens (unbalanced brackets)")?
            .to_string())
    }

    fn peek(&self) -> Result<String> {
        Ok(self.tokens
            .get(self.pos)
            .ok_or("Not enough tokens (unbalanced brackets)")?
            .to_string())
    }
}

pub fn tokenize(str: &str) -> Result<Vec<String>> {

    lazy_static! {
        static ref PARSE_RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]+)"###
        )
        .unwrap();
    }

    let mut res = vec![];

    for cap in PARSE_RE.captures_iter(str) {
        if cap[1].starts_with(";") { // comments
            continue;
        }
        res.push(String::from(&cap[1]));
    }

    Ok(res)
}

fn read_atomic(reader: &mut Reader) -> Result<LangVal> {

    lazy_static! {
        static ref NUM_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
        static ref STR_RE: Regex = Regex::new(r#""(?:\\.|[^\\"])*""#).unwrap();
    }

    let token = reader.next()?;

    // cases
    if token == "nil" {
        return Ok(LangVal::Nil)
    }
    if token == "true" {
        return Ok(LangVal::Boolean(true));
    }
    if token == "false" {
        return Ok(LangVal::Boolean(false));
    }
    if token.starts_with(":") {
        return Ok(LangVal::String(token));
    }
    if NUM_RE.is_match(&token) {
        return Ok(LangVal::Number(token.parse()?));
    }
    if STR_RE.is_match(&token) {
        return Ok(LangVal::String(token)); // TODO: parse the newlins, etc.
    }
    if token.starts_with("\"") {
        Err("Unexpected \" (unbalanced string literal)")?;
    }

    Ok(LangVal::Symbol(token))
}

fn make_hashmap(tokens: Vec<LangVal>) -> Result<LangVal> {

    if tokens.len() % 2 != 0 {
        Err("Invalid size hashmap")?;
    }

    let mut mp: HashMap<String, LangVal> = Default::default();

    for (k, v) in tokens.into_iter().tuples() {
        match k {
            LangVal::Number(n) => {
                mp.insert(n.to_string(), v.clone());
            }
            LangVal::String(s) => {
                mp.insert(s, v.clone());
            }
            _ => {
                Err("Invalid hashmap key type")?
            }
        }
    }

    Ok(LangVal::Hashmap(mp))
}

fn read_list(reader: &mut Reader, end: &str) -> Result<LangVal> {
    let mut ret: Vec<LangVal> = vec![];
    reader.next()?;

    loop {
        if reader.peek()? == end {
            break;
        }

        ret.push(read_form(reader)?);
    }

    reader.next()?;

    match end {
        ")" => Ok(LangVal::List(ret)),
        "]" => Ok(LangVal::Vector(ret)),
        "}" => make_hashmap(ret),
        _ => Err("Unknown ending")?
    }
}

fn read_form(reader: &mut Reader) -> Result<LangVal>{
    let token = reader.peek()?;

    match token.as_str() {
        "'" => {
            reader.next()?;
            Ok(LangVal::WithSpecial(("quote".to_string(), Rc::new(read_form(reader)?))))
        }
        "`" => {
            reader.next()?;
            Ok(LangVal::WithSpecial(("quasiquote".to_string(), Rc::new(read_form(reader)?))))
        }
        "~" => {
            reader.next()?;
            Ok(LangVal::WithSpecial(("unquote".to_string(), Rc::new(read_form(reader)?))))
        }
        "~@" => {
            reader.next()?;
            Ok(LangVal::WithSpecial(("splice-unquote".to_string(), Rc::new(read_form(reader)?))))
        }
        "@" => {
            reader.next()?;
            Ok(LangVal::WithSpecial(("deref".to_string(), Rc::new(read_form(reader)?))))
        }
        "^" => {
            reader.next()?;
            let meta = read_form(reader)?;
            Ok(LangVal::List(vec![LangVal::Symbol("with-meta".to_string()), read_form(reader)?, meta]))
        }
        ")" => Err("Unexpected ')'")?,
        "(" => read_list(reader, ")"),
        "]" => Err("Unexpected ']'")?,
        "[" => read_list(reader, "]"),
        "}" => Err("Unexpected '}'")?,
        "{" => read_list(reader, "}"),
        _ => read_atomic(reader)
    }
}

pub fn read_str(str: &str) -> Result<LangVal>{
    let tokens = tokenize(str)?;

    //println!("{:?}", tokens);

    Ok(read_form(&mut Reader {
        tokens,
        pos: 0
    })?)
}
