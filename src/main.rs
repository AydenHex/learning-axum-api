use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod handlers;
mod models;

#[tokio::main]
async fn main() {
    //Load env var
    dotenv().ok();

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        std::env::var("DATABASE_USER").expect("set DATABASE_USER env variable"),
        std::env::var("DATABASE_PASSWORD").expect("set DATABASE_PASSWORD env variable"),
        std::env::var("DATABASE_HOST").expect("set DATABASE_HOST env variable"),
        std::env::var("DATABASE_PORT").expect("set DATABASE_PORT env variable"),
        std::env::var("DATABASE_NAME").expect("set DATABASE_NAME env variable"),
    );

    // Init tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "leaning_axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cors = CorsLayer::new().allow_origin(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Can't connect to database");

    let app = Router::new()
        .route("/", get(|| async { "hello, world" }))
        .route("/register", post(handlers::auth::register))
        .layer(cors)
        .layer(Extension(pool));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
