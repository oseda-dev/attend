use std::error::Error;

use axum::{Json, Router, routing::{get, post}};
use tokio::net::TcpListener;



#[derive(serde::Deserialize, Debug)]
struct LogAttendanceRequest {
    // need to handle a session ID eventually
    id: String,
    email: String,
}

pub async fn serve_backend() -> Result<(), Box<dyn Error + Send + Sync>> {

    let app: Router = Router::new()
        .route("/record", post(record_attendance));
        
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())

}

async fn record_attendance(Json(payload): Json<LogAttendanceRequest>) {
    println!("{:?}", payload);
}
