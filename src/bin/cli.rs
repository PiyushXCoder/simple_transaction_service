use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Args {
    #[arg(short, long, default_value_t = false, help = "Run database migrations")]
    migrate: bool,

    #[arg(short, long, help = "Database connection URL")]
    postgres_url: Option<String>,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let postgres_url = args.postgres_url.unwrap_or(
        env::var("DATABASE_URL")
            .expect("Database URL must be set in the environment or via --postgres-url"),
    );

    if args.migrate {
        let migrator = sqlx::migrate!("./migrations");
        let pool = PgPoolOptions::new()
            .connect(&postgres_url)
            .await
            .expect("Failed to connect to the database");
        migrator.run(&pool).await.expect("Failed to run migrations");
        println!("Database migrations applied successfully.");
    }
}
