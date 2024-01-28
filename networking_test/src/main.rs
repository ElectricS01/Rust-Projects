use axum::{
    routing::{get, post},
    response::Html,
    Router,
};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let request_count = Arc::new(Mutex::new(0));

    let app = Router::new()
        .route("/", get({
            let request_count = Arc::clone(&request_count);
            move || {
                async move {
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
            }
        }))
        .route("/request", post({
            let request_count = Arc::clone(&request_count);
            move || {
                let request_count = Arc::clone(&request_count);
                async move {
                    let mut request_count = request_count.lock().unwrap();
                    *request_count += 1;
                    Html(format!("{}", *request_count))
                }
            }
        }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

