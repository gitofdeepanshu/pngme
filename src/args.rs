use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Encode a png file
    Encode(Info),

    /// Decodes the message hidden inside the given op_code for a png file
    Decode(Info),

    /// Removes a given chunk from a png file
    Remove(Info),

    /// Prints the whole Png File
    Print(Info),
}

#[derive(Args)]
pub struct Info {
    pub location: String,
    pub op_code: Option<String>,
    pub message: Option<String>,
    pub output: Option<String>,
}
