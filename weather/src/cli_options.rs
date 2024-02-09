use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct LocationArgs {
    /// Name of the person to greet
    #[arg(short, long)]
    pub city: Option<String>,

    #[arg(short, long)]
    pub state: Option<String>,

    #[arg(long)]
    pub street: Option<String>

}