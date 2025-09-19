use clap::Parser;
use simple_transaction_service::start_server;

#[derive(Parser, Debug)]
#[command(version, about = "Simple Transction Service", long_about = None)]
struct Args {
    #[arg(
        short,
        long,
        help = "Database connection URL",
        default_value = "0.0.0.0:8000"
    )]
    bind_address: String,

    #[arg(short, long, help = "Database connection URL")]
    postgres_url: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    start_server(&args.bind_address)
        .await
        .expect("Failed to start server");
}
