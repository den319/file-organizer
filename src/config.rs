use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Config {
    pub path: String,

    #[arg(short, long)]
    pub dry_run: bool,
}