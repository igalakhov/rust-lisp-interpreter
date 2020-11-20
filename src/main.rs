#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rustyline;
mod reader;
mod types;
mod printer;

use rustyline::error::ReadlineError;
use rustyline::Editor;


#[tokio::main]
pub async fn main() {

    // main input/output loop
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("user> ");

        match readline {
            Ok(line) => {
                if line.len() == 0 {
                    continue;
                }

                match reader::read_str(line.as_str()) {
                    Ok(val) => {
                        printer::print_val(&val);
                    }
                    Err(why) => {
                        println!("Error while parsing: {:?}", why);
                    }
                }
            }
            Err(ReadlineError::Interrupted) => break, // also exit on ctrl-c
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Input error: {:?}", err);
                break;
            }
        }
    }
}
