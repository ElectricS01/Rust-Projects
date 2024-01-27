use axum::{
    routing::{get, post},
    response::Html,
    Router,
};

#[tokio::main]
async fn main() {
    let mut request_count: u32 = 0;

    let app = Router::new()
        .route("/", get(move || {
            println!("home");
            let request_count = request_count;
            async move {
                Html(format!(
                    r#"<h1>Welcome to TPUv6/TPUvRust</h1>
                    <p>Way better than TPUv5</p>
                    <p>Request count: {}</p>
                    <form action="/request" method="post">
                        <button name="foo" value="upvote">Upvote</button>
                    </form>"#,
                request_count
            ))
        }
        }))
        .route("/request", post(move || {
            request_count += 1;
            println!("request {}", request_count);
            async move {
                format!("{}", request_count);
            }
        }));
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
