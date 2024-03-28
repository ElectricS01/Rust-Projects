use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&(database_url + ":password@localhost/test"))
        .await;

    /* let mut email = "Email";

    let mut rows = sqlx::query("SELECT * FROM Users WHERE email = ?")
        .bind(email)
        .fetch(&mut pool); */

    let request_count = Arc::new(Mutex::new(0));

    let app = Router::new()
        .route(
            "/",
            get({
                let request_count = Arc::clone(&request_count);
                move || async move {
                    let mut request_count = request_count.lock().unwrap();
                    *request_count += 1;
                    Html(format!(
                        r#"<h1>Welcome to TPUv6/TPUvRust</h1>
                        <p>Way better than TPUv5</p>
                        <p>Request count: {}</p>
                        <form action="/request" method="post">
                            <button name="foo" value="upvote">Upvote</button>
                        </form>"#,
                        *request_count
                    ))
                }
            }),
        )
        .route(
            "/request",
            post({
                let request_count = Arc::clone(&request_count);
                move || {
                    let request_count = Arc::clone(&request_count);
                    async move {
                        let mut request_count = request_count.lock().unwrap();
                        *request_count += 1;
                        Html(format!("{}", *request_count))
                    }
                }
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
