use axum::{routing::get, Router};

async fn hello() -> &'static str {
    "Hello, I just created my first axum project!"
}

#[tokio::main]
pub async fn run() {
    let app = Router::new().route("/hello", get(hello));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server listened and its running on your local machine at http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
