use clap::Parser;
use rs_js_eval_boa::JsContext;
use std::io::Read;
use std::{io, process};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// JavaScript expression to evaluate
    #[arg(short, long)]
    expression: Option<String>,
}

fn main() {
    let args = Args::parse();

    let js_code = if let Some(expr) = args.expression {
        expr
    } else {
        // No expression, read from stdin
        let mut buffer = String::new();
        match io::stdin().read_to_string(&mut buffer) {
            Ok(_) => buffer,
            Err(e) => {
                eprintln!("Error reading from stdin: {}", e);
                process::exit(1);
            }
        }
    };

    let mut context = match JsContext::new() {
        Ok(context) => context,
        Err(e) => {
            eprintln!("Error initializing JS context: {}", e);
            process::exit(1);
        }
    };

    match context.evaluate(&js_code) {
        Ok(js_value) => {
            match context.val2json(&js_value) {
                Ok(Some(json_value)) => {
                    println!("{}", json_value);
                }
                Ok(None) => {
                    // No output if the JS value converts to JSON null (e.g. undefined)
                }
                Err(e) => {
                    eprintln!("Error converting JS value to JSON: {}", e);
                    process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("JavaScript Error: {}", e);
            process::exit(1);
        }
    }
}
