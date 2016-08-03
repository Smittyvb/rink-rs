extern crate rink;
#[cfg(feature = "rustyline")]
extern crate rustyline;

use rink::*;

#[cfg(feature = "rustyline")]
fn main() {
    use rustyline::error::ReadlineError;
    use rustyline::Editor;

    let mut rl = Editor::new();
    let mut ctx = match load() {
        Ok(ctx) => ctx,
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                match one_line(&mut ctx, &*line) {
                    Ok(v) => ctx.print(&v),
                    Err(e) => println!("{}", e)
                };
            },
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Readline: {:?}", err);
                break
            },
        }
    }
}

#[cfg(not(feature = "rustyline"))]
fn main() {
    use std::io::{stdin, stdout, Write};

    let mut ctx = match load() {
        Ok(ctx) => ctx,
        Err(e) => {
            println!("{}", e);
            return
        }
    };
    let f = stdin();
    let mut line = String::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();
        match f.read_line(&mut line) {
            Ok(_) => (),
            Err(_) => return
        };
        match one_line(&mut ctx, &*line) {
            Ok(v) => ctx.print(&v),
            Err(e) => println!("{}", e)
        };
        line.clear();
    }
}