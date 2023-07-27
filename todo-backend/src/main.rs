use std::net::SocketAddr;

use axum::{
    Router,
    routing::get, extract::{State, Path}, Json, Form
};
use axum_error::Result;
use serde::{Serialize, Deserialize};
use sqlx::SqlitePool;
use tower_http::cors::CorsLayer;

#[derive(Serialize, Deserialize)]
struct Todo {
    id: i64,
    description: String,
    done: bool,
}

#[derive(Serialize, Deserialize)]
struct NewTodo {
    description: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Get environmental variables
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    // Create router for server
    let app = Router::new()
        .route("/", get(list))
        .route("/create", get(create))
        .route("/delete/:id", get(delete))
        .route("/update", get(update))
        .with_state(pool)
        .layer(CorsLayer::very_permissive());

    let address = SocketAddr::from(([0, 0, 0, 0], 8181));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

async fn list(
    State(pool): State<SqlitePool>
) -> Result<Json<Vec<Todo>>> {
    // List all todos
    let todos = sqlx::query_as!(
        Todo, "SELECT id, description, done FROM todos ORDER BY id"
    )
        .fetch_all(&pool)
        .await?;
    Ok(Json(todos))
}

async fn create(
    State(pool): State<SqlitePool>,
    Form(todo): Form<NewTodo>
) -> Result<String> {
    // Create TODO
    sqlx::query!(
        "INSERT INTO todos (description) VALUES (?)",
        todo.description
    )
        .execute(&pool)
        .await?;

    Ok(format!("Successfully inserted TODO!"))
}

async fn delete(
    State(pool): State<SqlitePool>,
    Path(id): Path<i64>
) -> Result<String> {
    // Create TODO
    sqlx::query!(
        "DELETE FROM todos where id = ?",
        id
    )
        .execute(&pool)
        .await?;

    Ok(format!("Successfully deleted TODO!"))
}

async fn update(
    State(pool): State<SqlitePool>,
    Form(todo): Form<Todo>
) -> Result<String> {
    // Create TODO
    sqlx::query!(
        "UPDATE todos SET description = ?, done = ? WHERE id = ?",
        todo.description,
        todo.done,
        todo.id,
    )
        .execute(&pool)
        .await?;

    Ok(format!("Successfully updated TODO!"))
}