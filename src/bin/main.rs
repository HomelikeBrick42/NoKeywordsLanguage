use std::io::Write;

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
            todo!()
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
