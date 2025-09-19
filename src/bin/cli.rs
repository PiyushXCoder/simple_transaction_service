use clap::Parser;
use simple_transaction_service::{db::api_keys::ApiKeys, sqlx_db_impl::SqlxDbStore};
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Args {
    #[arg(short, long, default_value_t = false, help = "Run database migrations")]
    migrate: bool,

    #[arg(short, long, default_value_t = false, help = "Run database migrations")]
    add_api_key: bool,

    #[arg(short, long, default_value_t = false, help = "Run database migrations")]
    list_api_keys: bool,

    #[arg(short, long, help = "Database connection URL")]
    postgres_url: Option<String>,
}

fn get_database_url(args: Args) -> String {
    args.postgres_url.unwrap_or(
        env::var("DATABASE_URL")
            .expect("Database URL must be set in the environment or via --postgres-url"),
    )
}

async fn get_postgres_pool(postgres_url: &str) -> sqlx::Pool<sqlx::Postgres> {
    PgPoolOptions::new()
        .connect(postgres_url)
        .await
        .expect("Failed to connect to the database")
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.migrate {
        let pool = get_postgres_pool(&get_database_url(args)).await;
        let migrator = sqlx::migrate!("./migrations");
        migrator.run(&pool).await.expect("Failed to run migrations");
        println!("Database migrations applied successfully.");
    } else if args.add_api_key {
        let db = SqlxDbStore::new(get_postgres_pool(&get_database_url(args)).await);
        let new_key = db
            .create_api_key()
            .await
            .expect("Failed to create new API key");

        println!("New API key added: {}", new_key);
    } else if args.list_api_keys {
        let db = SqlxDbStore::new(get_postgres_pool(&get_database_url(args)).await);
        let keys = db.list_api_keys().await.expect("Failed to list API keys");
        println!("API Keys:");
        for key in keys {
            println!("- {}", key);
        }
    }
}
