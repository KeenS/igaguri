use ast::{Input, Ast};
use nom::{multispace, IResult};
use std::str;
use rustyline::Editor;
use rustyline::error::ReadlineError;

pub struct Reader {
    rl: Editor<()>,
    histfile: String,
}


named!(token< String >, map!(map_res!(is_a!("abcdefghijklmnopqlrstuvwxyzABCDEFGHIJKLMNOPQLRSTUVWXYZ-!$%^&@/1234567890"), str::from_utf8), |i: &str| i.to_string()));

named!(pipe, chain!(opt!(multispace) ~ tag!("|") ~ opt!(multispace), || b"|"));
named!(command<Ast>, map!(separated_list!(multispace, token), Ast::Command));
named!(parse<Ast>, map!(separated_list!(pipe, command), Ast::Pipe));


impl Reader {
    pub fn new() -> Self {
        let mut reader = Reader {
            rl: Editor::<()>::new(),
            histfile: ".igaguri_history".to_string(),
        };
        reader.rl.load_history(&reader.histfile);
        reader
    }

    pub fn readline(&mut self) -> Result<Input, String> {
        let mut buffer = String::new();
        loop {
            let readline = self.rl.readline(">> ");
            match readline {
                Ok(input) => {
                    self.rl.add_history_entry(&input);
                    buffer = buffer + &input;
                    match parse(&buffer.as_bytes()) {
                        IResult::Done(_, ast) => {
                            debug!("Line: {}", input);
                            return Ok(Input::Input(ast));
                        }
                        IResult::Error(e) => {
                            error!("Error: {:?}", e);
                            return Err("parse failed".to_string());
                        }
                        IResult::Incomplete(_) => continue,
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    debug!("CTRL-C");
                    return Ok(Input::Interupt);
                }
                Err(ReadlineError::Eof) => {
                    debug!("CTRL-D");
                    return Ok(Input::Eof);
                }
                Err(err) => {
                    error!("Error: {:?}", err);
                    return Err("IO error".to_string());
                }
            }
        }
    }

    pub fn save_history(&self) -> Result<(), String> {
        self.rl.save_history(&self.histfile).map_err(|_| "failed to save the history".to_string())
    }
}
