use hell_o::create_app;

#[tokio::main]
async fn main() {
    let app = create_app();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind listener");
    println!("Server listened and its running on your local machine at http://localhost:3000");
    axum::serve(listener, app)
        .await
        .expect("failed to start server")
}
