use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let router = Router::new().route("/", get(say_hello));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn say_hello() -> String {
    "Hello, World!".to_string()
}
