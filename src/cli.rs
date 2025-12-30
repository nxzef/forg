use clap::Parser;

#[derive(Parser, Debug)]
pub struct Cli {
    #[clap(short, long, default_value = ".")]
    pub path: String,
    #[clap(long)]
    pub dry_run: bool,
}
