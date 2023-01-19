mod args;

use std::fs;

use clap::Parser;

use args::Args;
use markup::compile;

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Use default for output file
    let out = args
        .out
        // Replace file extension with 'html'
        .unwrap_or(replace_file_extension(&args.file, "html"));

    // Read file
    let file_in = fs::read_to_string(args.file).expect("Could not read input file");

    // Compile
    let file_out = compile(&file_in).expect("Failed to compile");

    // Write file
    fs::write(out, file_out).expect("Could not write output file");
}

/// Change filename extension to another string
///
/// All characters after last dot are included in extension
///
/// All characters (including dots), except everything after last dot, are included in filename
fn replace_file_extension(filename: &str, extension: &str) -> String {
    let mut split_at_dot = filename.split('.');

    let last = split_at_dot.next_back().unwrap_or("");
    let rest = split_at_dot.collect::<Vec<_>>();

    let filename = if rest.is_empty() {
        last.to_string()
    } else {
        rest.join(".")
    };

    if extension.is_empty() {
        return filename;
    }

    filename + "." + extension
}
