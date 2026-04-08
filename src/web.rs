use core::{fmt, time};
use std::{collections::HashMap, error::Error, fs::{File, OpenOptions}, os::unix::net::SocketAddr};

use axum::{Json, Router, extract::State, http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}};
use derive_more::derive;
use tokio::net::TcpListener;
use std::io::Write;

use crate::{AttendFrontendState, templates::{self, HTML}};


#[derive(serde::Deserialize, Debug)]
struct LogAttendanceRequest {
    // need to handle a session ID eventually
    id: String,
    email: String,
}


#[derive(Clone, Debug)]
pub struct Socket {
    pub ip: String,
    pub port: u16
}

impl fmt::Display for Socket{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

pub async fn serve(state: &AttendFrontendState) -> Result<(), Box<dyn Error + Send + Sync>> {

    let app: Router = Router::new()
        .route("/", get(frontend))
        .route("/record", post(record_attendance))
        .with_state(state.clone());

    let listener = TcpListener::bind(state.bind_socket.to_string()).await?;
    axum::serve(listener, app).await?;

    Ok(())

}

// pretty much just a wrapper. This cant return a result for the axum::route function
async fn frontend(State(state): State<AttendFrontendState>) -> impl IntoResponse {
    match render_frontend(state.date, state.pub_socket) {
        Ok(html_content) => (StatusCode::OK, Html(html_content)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template Error: {}", err),
        ).into_response(),
    }
}

fn render_frontend(date: String, socket: Socket) -> Result<String, Box<dyn std::error::Error>> {
    let template = templates::load_template()?;
    
    let mut replacements: HashMap<String, String> = HashMap::new();
    replacements.insert("DATE".to_owned(), date);
    replacements.insert("SOCKET".to_owned(), socket.to_string());
    
    let output = templates::render_template(template, replacements);
    
    println!("{:?}", output.clone());
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
