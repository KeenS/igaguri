use ast::{Input, Ast};
use std::io::{self, ErrorKind};
use std::process::{Command, Child, Stdio};
use std::os::unix::io::{FromRawFd, IntoRawFd};
use read::Reader;


pub struct Igaguri {
    reader: Reader,
}

impl Igaguri {
    pub fn new() -> Self {
        Igaguri { reader: Reader::new() }
    }

    pub fn repl(&mut self) {
        loop {
            let readline = self.reader.readline();
            match readline {
                Ok(Input::Input(ast)) => {
                    debug!("Line: {:?}", ast);
                    if !ast.is_empty() {
                        match self.run(ast, Stdio::inherit(), Stdio::inherit(), Stdio::inherit()) {
                            Ok(mut child) => {
                                child.wait();
                            }
                            Err(e) => {
                                debug!("{}", e);
                            }
                        }
                    }
                }
                Ok(Input::Interupt) => {
                    debug!("C-c");
                }
                Ok(Input::Eof) => {
                    debug!("eof");
                    break;
                }
                Err(s) => {
                    error!("{}", s);
                }
            }
        }
        self.reader.save_history().unwrap();

    }

    fn run(&mut self, ast: Ast, stdin: Stdio, stdout: Stdio, stderr: Stdio) -> io::Result<Child> {
        match ast {
            Ast::Command(mut terms) => {
                debug!("{:?}", terms);
                if terms.len() < 1 {
                    return Err(io::Error::new(ErrorKind::InvalidInput, "input is not a command"));
                }
                let cmd = terms.drain(0..1).next().unwrap();
                Command::new(cmd)
                    .args(&terms)
                    .stdin(stdin)
                    .stdout(stdout)
                    .stderr(stderr)
                    .spawn()
            }
            Ast::Pipe(commands) => {
                debug!("{:?}", commands);
                let mut si = stdin;
                let mut itr = commands.into_iter().peekable();
                unsafe {
                    while let Some(command) = itr.next() {
                        if itr.peek().is_some() {
                            let process =
                                try!(self.run(command, si, Stdio::piped(), Stdio::inherit()));
                            si = Stdio::from_raw_fd(process.stdout.unwrap().into_raw_fd());
                        } else {
                            return self.run(command, si, stdout, stderr);
                        }
                    }
                }
                unreachable!()
            }
        }
    }
}
