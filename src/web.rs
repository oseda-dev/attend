use core::{fmt, time};
use std::{collections::HashMap, error::Error, fs::{File, OpenOptions}, os::unix::net::SocketAddr};

use axum::{Json, Router, http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}};
use tokio::net::TcpListener;
use std::io::Write;

use crate::templates::{self, HTML};


#[derive(serde::Deserialize, Debug)]
struct LogAttendanceRequest {
    // need to handle a session ID eventually
    id: String,
    email: String,
}


pub struct Socket {
    pub ip: String,
    pub port: u16
}

impl fmt::Display for Socket{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

pub async fn serve(sock: &Socket) -> Result<(), Box<dyn Error + Send + Sync>> {

    let app: Router = Router::new()
        .route("/", get(frontend))
        .route("/record", post(record_attendance));
        

    let listener = TcpListener::bind(sock.to_string()).await?;
    axum::serve(listener, app).await?;

    Ok(())

}

// pretty much just a wrapper. This cant return a result for the axum::route function
async fn frontend() -> impl IntoResponse {
    match render_frontend() {
        Ok(html_content) => (StatusCode::OK, Html(html_content)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template Error: {}", err),
        ).into_response(),
    }
}

fn render_frontend() -> Result<String, Box<dyn std::error::Error>> {
    let template = templates::load_template()?;
    
    let mut replacements: HashMap<String, String> = HashMap::new();
    replacements.insert("DATE".to_owned(), "2025".to_owned());
    
    let output = templates::render_template(template, replacements);
    
    Ok(output.to_string())
}


async fn record_attendance(Json(payload): Json<LogAttendanceRequest>) {
    println!("{:?}", payload);
    let _ = csv_add_row(&payload.email, &payload.id, "now").expect("Could not append row");
}

fn csv_add_row(email: &str, id: &str, timestamp: &str) -> Result<(), Box<dyn Error>> {
    // todo create file upon server start
    
    let mut file = OpenOptions::new()
        .append(true)
        .open("log.csv")?;


    let row = format!("{},{},{}", email, id, timestamp);
    writeln!(file, "{}", row)?;
    

    Ok(())
}
