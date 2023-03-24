use no_keywords_language::{
    binding::{bind_file, BoundNode, CommonTypes, Type},
    emit::emit_file,
    nodes::{NodeID, Nodes},
    parsing::parse_file,
    tokens::{GetLocation, Lexer, SourceLocation},
};
use std::{collections::HashMap, io::Write};

fn help(program_name: &str, f: &mut dyn Write) -> std::io::Result<()> {
    writeln!(f, "Usage: {program_name} {{command}} [options]")?;
    Ok(())
}

fn compile_ir<'filepath>(
    filepath: &'filepath str,
    stderr: &mut dyn Write,
) -> (
    Nodes<BoundNode<'filepath>>,
    Nodes<Type>,
    NodeID<BoundNode<'filepath>>,
    NodeID<BoundNode<'filepath>>,
) {
    let source = std::fs::read_to_string(filepath).unwrap_or_else(|e| {
        writeln!(stderr, "Unable to open '{filepath}': {e}").unwrap();
        std::process::exit(1)
    });
    let expressions = parse_file(filepath, &source).unwrap_or_else(|e| {
        writeln!(stderr, "{e}").unwrap();
        std::process::exit(1)
    });

    let builtin_location = SourceLocation {
        filepath: "builtin.nkl",
        position: 0,
        line: 1.try_into().unwrap(),
        column: 1.try_into().unwrap(),
    };

    let mut nodes = Nodes::new();
    let mut types = Nodes::new();

    let typ = types.insert(Type::Type);
    let type_node = nodes.insert(BoundNode::Type {
        location: builtin_location,
        end_location: builtin_location,
        typ,
        type_type: typ,
    });

    let void = types.insert(Type::Void);
    let void_node = nodes.insert(BoundNode::Type {
        location: builtin_location,
        end_location: builtin_location,
        typ: void,
        type_type: typ,
    });

    let u8 = types.insert(Type::U8);
    let u8_node = nodes.insert(BoundNode::Type {
        location: builtin_location,
        end_location: builtin_location,
        typ: u8,
        type_type: typ,
    });

    let int = types.insert(Type::Int);
    let int_node = nodes.insert(BoundNode::Type {
        location: builtin_location,
        end_location: builtin_location,
        typ: int,
        type_type: typ,
    });

    let uint = types.insert(Type::UInt);
    let uint_node = nodes.insert(BoundNode::Type {
        location: builtin_location,
        end_location: builtin_location,
        typ: uint,
        type_type: typ,
    });

    let mut names = HashMap::from([
        ("type", type_node),
        ("void", void_node),
        ("u8", u8_node),
        ("int", int_node),
        ("uint", uint_node),
    ]);

    let mut common_types = CommonTypes {
        typ,
        void,
        int,
        uint,
        u8,
        slice_types: HashMap::new(),
        pointer_types: HashMap::new(),
        multipointer_types: HashMap::new(),
        procedure_types: HashMap::new(),
    };

    let root = bind_file(
        filepath,
        &expressions,
        &mut nodes,
        &mut types,
        &mut names,
        &mut common_types,
    )
    .unwrap_or_else(|e| {
        writeln!(stderr, "{e}").unwrap();
        std::process::exit(1)
    });

    let multipointer_of_u8 = common_types.get_multipointer(&mut types, u8);
    let slice_of_multipointer_of_u8 = common_types.get_slice(&mut types, multipointer_of_u8);
    let main_procedure_type =
        common_types.get_procedure(&mut types, &[slice_of_multipointer_of_u8], int);

    let main_procedure = if let Some(&procedure_id) = names.get(&"main") {
        let procedure = &nodes[procedure_id];
        let procedure_type = procedure.get_type(&nodes);
        if main_procedure_type != procedure_type {
            writeln!(
                stderr,
                "Expected the main function to have the type {}, but got {}",
                types[main_procedure_type].pretty_print(&types),
                types[procedure_type].pretty_print(&types),
            )
            .unwrap();
            std::process::exit(1)
        }
        procedure_id
    } else {
        writeln!(stderr, "Expected a procedure called main").unwrap();
        std::process::exit(1)
    };

    (nodes, types, root, main_procedure)
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
            let filepath = args.next().unwrap_or_else(|| {
                writeln!(stderr, "Expected a source file to parse the ast from").unwrap();
                help(&program_name, stderr).unwrap();
                std::process::exit(1)
            });
            let source = std::fs::read_to_string(&filepath).unwrap_or_else(|e| {
                writeln!(stderr, "Unable to open '{filepath}': {e}").unwrap();
                std::process::exit(1)
            });
            let expressions = parse_file(&filepath, &source).unwrap_or_else(|e| {
                writeln!(stderr, "{e}").unwrap();
                std::process::exit(1)
            });
            for expression in expressions {
                writeln!(stdout, "{expression:#?}").unwrap();
            }
        }

        "dump_ir" => {
            let filepath = args.next().unwrap_or_else(|| {
                writeln!(stderr, "Expected a source file to parse the ast from").unwrap();
                help(&program_name, stderr).unwrap();
                std::process::exit(1)
            });

            let (nodes, types, root, main_procedure) = compile_ir(&filepath, stderr);
            _ = nodes;
            _ = types;
            _ = root;
            _ = main_procedure;
        }

        "emit_asm" => {
            let filepath = args.next().unwrap_or_else(|| {
                writeln!(stderr, "Expected a source file to parse the ast from").unwrap();
                help(&program_name, stderr).unwrap();
                std::process::exit(1)
            });

            let (nodes, types, root, main_procedure) = compile_ir(&filepath, stderr);
            let mut f = std::fs::File::options()
                .write(true)
                .create(true)
                .open("output.asm")
                .unwrap();
            emit_file(root, main_procedure, &nodes, &types, &mut f).unwrap();
        }

        _ => {
            writeln!(stderr, "Unknown command: '{command}'").unwrap();
            help(&program_name, stderr).unwrap();
            std::process::exit(1)
        }
    }
}
