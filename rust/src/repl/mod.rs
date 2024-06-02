use std::io::{BufRead, Read, Write};

use crate::{Lexer, Parser};

pub fn start_repl(read: impl Read, write: impl Write) {
    let mut reader = std::io::BufReader::new(read);
    let mut writer = std::io::BufWriter::new(write);
    loop {
        writer.write_all(b"> ").unwrap();
        writer.flush().unwrap();
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        let lexer = Lexer::new(line);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        match program {
            Ok(program) => writer.write_all(program.to_string().as_bytes()).unwrap(),
            Err(err) => {
                writer
                    .write_all(format!("ERROR: {err}").as_bytes())
                    .unwrap();
            },
        }
        writer.write_all(b"\n").unwrap();
    }
}
