pub use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    #[arg(short, long)]
    pub user_id: String,
}
