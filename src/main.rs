use axum::{
    extract::Extension,
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod handlers;
mod models;
mod settings;

use crate::settings::Settings;

#[tokio::main]
async fn main() {
    let settings = Settings::new().unwrap();

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        settings.database_user,
        settings.database_password,
        settings.database_host,
        settings.database_port,
        settings.database_name
    );

    // Init tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
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
