use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Input filepath
    pub file: String,

    /// Output filepath
    pub out: Option<String>,
}
