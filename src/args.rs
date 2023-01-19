use clap::Parser;

#[derive(Parser)]
#[clap(author, version)]
pub struct Args {
    /// Input filepath
    pub file: String,

    /// Output filepath
    #[arg(short, long)]
    pub out: Option<String>,
}
