use std::error::Error;

use axum::{Router, routing::get};
use tokio::net::TcpListener;



#[derive(serde::Deserialize)]
struct LogAttendanceRequest {
    // need to handle a session ID eventually
    id: String,
    email: String,
}

pub async fn serve_backend() -> Result<(), Box<dyn Error + Send + Sync>> {

    let app: Router = Router::new()
    .route("/", get(async || {
        "hello world"
        }));
        
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())

}
