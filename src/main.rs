mod boards;
mod keycodes;
mod parser;
mod render;

use boards::BOARDS;
use parser::{detect_macro, extract_layers};
use std::process;

fn is_digits(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_digit())
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let path = if !args.is_empty() && !is_digits(&args[0]) {
        args[0].clone()
    } else {
        "keymap.c".to_string()
    };

    let layer_args: Vec<usize> = if args.len() > 1 {
        args[1..].iter().map(|a| a.parse().unwrap()).collect()
    } else if !args.is_empty() && is_digits(&args[0]) {
        args.iter().map(|a| a.parse().unwrap()).collect()
    } else {
        Vec::new()
    };

    let src = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Could not read {}: {}", path, e);
            process::exit(1);
        }
    };

    let macro_names: Vec<&str> = BOARDS.iter().map(|b| b.macro_name).collect();
    let macro_name = match detect_macro(&src, &macro_names) {
        Some(m) => m,
        None => {
            let known = macro_names.join(", ");
            println!(
                "No supported LAYOUT_xxx(...) macro found in {}\n(known boards: {})",
                path, known
            );
            process::exit(1);
        }
    };
    let board = boards::find(macro_name).unwrap();

    let layers = extract_layers(&src, macro_name);
    if layers.is_empty() {
        println!("No {}(...) layers found in {}", macro_name, path);
        process::exit(1);
    }

    let wanted: Vec<usize> = if layer_args.is_empty() {
        layers.keys().copied().collect()
    } else {
        layer_args
    };

    for idx in wanted {
        match layers.get(&idx) {
            Some(tokens) => {
                println!("{}", (board.render)(idx, tokens));
                println!();
            }
            None => {
                println!("_{}\n  (not found)\n", idx);
            }
        }
    }
}
