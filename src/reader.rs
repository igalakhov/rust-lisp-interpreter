use regex::Regex;
use crate::types::{Result, LangVal};
use std::fs::read;

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
            .ok_or("Not enough tokens (next exhausted)")?
            .to_string())
    }

    fn peek(&self) -> Result<String> {
        Ok(self.tokens
            .get(self.pos)
            .ok_or("Not enough tokens (peek index out of range)")?
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
        if cap[1].starts_with(";") {
            continue;
        }
        res.push(String::from(&cap[1]));
    }

    Ok(res)
}

fn read_atomic(reader: &mut Reader) -> Result<LangVal> {

    lazy_static! {
        static ref NUM_RE: Regex = Regex::new(r"^-?[0-9]+$").unwrap();
    }

    let token = reader.next()?;

    if NUM_RE.is_match(&token) {
        return Ok(LangVal::Number(token.parse()?));
    }

    Ok(LangVal::Symbol(token))
}

fn read_list(reader: &mut Reader) -> Result<LangVal> {
    let mut ret: Vec<LangVal> = vec![];
    reader.next()?;

    loop {
        if reader.peek()? == ")" {
            break;
        }

        ret.push(read_form(reader)?);
    }

    reader.next();

    Ok(LangVal::List(ret))
}

fn read_form(reader: &mut Reader) -> Result<LangVal>{
    let token = reader.peek()?;

    match token.as_str() {
        ")" => Err("Unexpected ')'")?,
        "(" => read_list(reader),
        _ => read_atomic(reader)
    }
}

pub fn read_str(str: &str) -> Result<LangVal>{
    let tokens = tokenize(str)?;

    Ok(read_form(&mut Reader {
        tokens,
        pos: 0
    })?)
}
