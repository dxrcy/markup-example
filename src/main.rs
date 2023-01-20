mod args;

use std::fs;

use clap::Parser;

use args::Args;
use markup::{compile, replace_file_extension};

fn main() {
    // Parse CLI arguments
    let args = Args::parse();

    // Use default for output file
    let out = args
        .out
        // Replace file extension with 'html'
        .unwrap_or(replace_file_extension(&args.file, "html"));

    // Read file
    let file_contents = fs::read_to_string(args.file).expect("Could not read input file");
    // Compile
    let file_compiled = compile(&file_contents);
    // Write file
    fs::write(out, file_compiled).expect("Could not write output file");
}
