#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate rustyline;
mod reader;
mod types;
mod printer;
mod eval;
mod core;

use rustyline::error::ReadlineError;
use rustyline::Editor;


#[tokio::main]
pub async fn main() {

    // main input/output loop
    let mut rl = Editor::<()>::new();

    let core_env = core::make_core_env();

    loop {
        let readline = rl.readline("user> ");

        match readline {
            Ok(line) => {
                if line.len() == 0 {
                    continue;
                }

                match reader::read_str(line.as_str()) {
                    Ok(val) => {
                        match eval::eval(val, core_env.clone()) {
                            Ok(res) => {
                                printer::print_val(&res);
                            }
                            Err(why) => {
                                println!("Error during evaluation: {:?}", why);
                            }
                        }
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
