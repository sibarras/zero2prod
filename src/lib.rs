use axum::routing::get;

pub async fn run() -> Result<(), std::io::Error> {
    let app = axum::Router::new().route("/", get(|| async { "Hello World!" }));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}
