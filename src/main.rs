mod env;
mod files;
mod server;

use axum::{routing::get, Router};
use dotenv::dotenv;
use std::net::SocketAddr;

use server::{read_file, serve_files};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = Router::new()
        .route("/tree/", get(|| serve_files(None)))
        .route("/tree/:path", get(serve_files))
        .route("/blob/", get(|| read_file(None)))
        .route("/blob/:path", get(read_file));

    let addr: SocketAddr = "0.0.0.0:8000".parse().expect("Address parsing");

    println!("Server is listening at {}", addr.to_string());

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Server listening");
}
