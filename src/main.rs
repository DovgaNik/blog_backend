use axum::{extract::State, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, Pool, Postgres};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://root:root@localhost/postgres")
        .await
        .expect("Error connecting to database");

    let shared_state = AppState {
        db_pool: Arc::new(pool),
    };

    let app = Router::new().route("/", get(root)).with_state(shared_state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    db_pool: Arc<Pool<Postgres>>, // Shared pool
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Post {
    id: i32,
    title: String,
    text: String,
    date: DateTime<Utc>,
}

async fn root(State(state): State<AppState>) -> Json<Vec<Post>> {
    let db_pool = &state.db_pool;

    let post = sqlx::query_as::<_, Post>("SELECT * FROM blog.post")
        .fetch_all(&**db_pool)
        .await
        .expect("Error fetching post");

    Json(post)
}
