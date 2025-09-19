use clap::Parser;
use simple_transaction_service::start_server;
use std::env;

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

fn get_database_url(postgres_url: Option<String>) -> String {
    postgres_url.unwrap_or(
        env::var("DATABASE_URL")
            .expect("Database URL must be set in the environment or via --postgres-url"),
    )
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    start_server(&args.bind_address, &get_database_url(args.postgres_url))
        .await
        .expect("Failed to start server");
}
