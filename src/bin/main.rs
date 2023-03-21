use std::io::Write;

use no_keywords_language::tokens::{GetLocation, Lexer};

fn help(program_name: &str, f: &mut dyn Write) -> std::io::Result<()> {
    writeln!(f, "Usage: {program_name} {{command}} [options]")?;
    Ok(())
}

fn main() {
    let stdout = &mut std::io::stdout();
    let stderr = &mut std::io::stderr();

    let mut args = std::env::args();
    let program_name = args
        .next()
        .expect("The program name should be the first argument");

    let command = args.next().unwrap_or_else(|| {
        help(&program_name, stderr).unwrap();
        std::process::exit(1)
    });
    match command.as_str() {
        "help" => {
            help(&program_name, stdout).unwrap();
        }

        "dump_tokens" => {
            let filepath = args.next().unwrap_or_else(|| {
                writeln!(stderr, "Expected a source file to lex the tokens from").unwrap();
                help(&program_name, stderr).unwrap();
                std::process::exit(1)
            });
            let source = std::fs::read_to_string(&filepath).unwrap_or_else(|e| {
                writeln!(stderr, "Unable to open '{filepath}': {e}").unwrap();
                std::process::exit(1)
            });
            let lexer = Lexer::new(&filepath, &source);
            for token in lexer {
                match token {
                    Ok(token) => writeln!(stdout, "{}: {token}", token.get_location()).unwrap(),
                    Err(e) => writeln!(stdout, "{e}").unwrap(),
                }
            }
        }

        "dump_ast" => {
            todo!()
        }

        "dump_ir" => {
            todo!()
        }

        "run" => {
            todo!()
        }

        _ => {
            writeln!(stderr, "Unknown command: '{command}'").unwrap();
            help(&program_name, stderr).unwrap();
            std::process::exit(1)
        }
    }
}
