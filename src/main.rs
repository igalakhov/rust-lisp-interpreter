extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;


fn main() {
    // main input/output loop
    let mut rl = Editor::<()>::new();

    loop {
        let readline = rl.readline("user> ");

        match readline {
            Ok(line) => {
                if line.len() == 0 {
                    continue;
                }

                println!("{}", line);
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
