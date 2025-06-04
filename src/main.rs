use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    server: bool,
    #[arg(short, long, default_value = "127.0.0.1")]
    ip: String,
    #[arg(short, long, default_value = "8080")]
    port: u16,
    #[arg(long, default_value = "0")]
    display: u32,
}

fn main() {
    let _args = Args::parse();
    println!("CLI is not implemented in test environment");
}
