use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[clap(short, long, default_value = "3000")]
    pub polling_interval_ms: u64,
}
