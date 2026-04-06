use core::time;
use std::{error::Error, fs::{File, OpenOptions}};

use axum::{Json, Router, routing::{get, post}};
use tokio::net::TcpListener;
use std::io::Write;


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
    let _ = csv_add_row(&payload.email, &payload.id, "now").expect("Could not append row");
}

fn csv_add_row(email: &str, id: &str, timestamp: &str) -> Result<(), Box<dyn Error>> {
    
    let mut file = OpenOptions::new()
        .append(true)
        .open("log.csv")?;


    let row = format!("{},{},{}", email, id, timestamp);
    writeln!(file, "{}", row)?;
    

    Ok(())
}
