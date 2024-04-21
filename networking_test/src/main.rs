use async_std::task;
use axum::{
    response::Html,
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::types::chrono::{self, DateTime};
use sqlx::{Error, FromRow, MySql, MySqlPool, Pool};
use std::process::exit;
use std::sync::{Arc, Mutex};

#[derive(FromRow)]
struct Embed {
    embedLink: String,
    securityToken: String,
}

#[derive(FromRow)]
struct Message {
    id: i32,
    userId: i32,
    messageContents: String,
    embeds: Option<String>,
    edited: bool,
    reply: Option<i32>,
    chatId: i32,
    pinned: bool,
    createdAt: DateTime<chrono::Utc>,
    updatedAt: DateTime<chrono::Utc>,
}

async fn connect() -> Result<Pool<MySql>, Error> {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let database_user = std::env::var("DATABASE_USER").expect("DATABASE_USER must be set.");
    let database_password =
        std::env::var("DATABASE_PASSWORD").expect("DATABASE_PASSWORD must be set.");

    return MySqlPool::connect(
        &("mysql://".to_owned() + &database_user + ":" + &database_password + "@" + &database_url),
    )
    .await;
}

async fn do_run_query() -> Result<Vec<Message>, String> {
    let result = task::block_on(connect());

    match result {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
            exit(1);
            // return Err("Cannot query database".to_owned());
        }

        Ok(pool) => {
            let query_result = sqlx::query_as::<_, Message>("select * from messages where id = 1")
                .fetch_all(&pool)
                .await
                .unwrap();

            println!("Number of messages selected: {}", query_result.len());

            for (rindex, message) in query_result.iter().enumerate() {
                println!(
                    "{},  {}, {}, {}, {}, {}, {}, {}, {}",
                    rindex + 1,
                    &message.id,
                    &message.userId,
                    &message.messageContents,
                    &message.edited,
                    &message.chatId,
                    &message.pinned,
                    &message.createdAt,
                    &message.updatedAt
                );
            }

            return Ok(query_result);
        }
    }
}

#[tokio::main]
async fn main() {
    let result = task::block_on(do_run_query());

    match result {
        Err(err) => {
            println!("Cannot connect to database [{}]", err.to_string());
            exit(1);
        }

        Ok(res) => {
            let request_count = Arc::new(Mutex::new(0));
            let mes = res[0].messageContents.clone();

            let app = Router::new()
                .route(
                    "/",
                    get({
                        let request_count = Arc::clone(&request_count);
                        move || async move {
                            let mut request_count = request_count.lock().unwrap();
                            let mes = mes.clone();
                            *request_count += 1;
                            Html(format!(
                                r#"<h1>Welcome to TPUv6/TPUvRust</h1>
                        <p>Way better than TPUv5</p>
                        <p>Request count: {}</p>
                        <p>{}</p>
                        <form action="/request" method="post">
                            <button name="foo" value="upvote">Upvote</button>
                        </form>"#,
                                *request_count, mes
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
    }
}
