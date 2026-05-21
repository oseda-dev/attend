use core::fmt;
use std::{collections::HashMap, error::Error, fs::OpenOptions, path::PathBuf};

use axum::{Json, Router, extract::State, http::StatusCode, response::{Html, IntoResponse}, routing::{get, post}};
use std::io::Write;

use crate::{AttendFrontendState, templates::{self}};


#[derive(serde::Deserialize, Debug)]
struct LogAttendanceRequest {
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


    let addr = state.bind_socket.to_string().parse::<std::net::SocketAddr>()?;
    let socket = tokio::net::TcpSocket::new_v4()?;

    socket.set_reuseaddr(true)?; 

    socket.bind(addr)?;
    let listener = socket.listen(1024)?;


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
    
    Ok(output.to_string())
}


async fn record_attendance(
    State(state): State<AttendFrontendState>,
    Json(payload): Json<LogAttendanceRequest>) {

    let log_path: PathBuf = [
            state.home_path, state.class.into(), state.date.into(), "log.csv".into()
        ].iter().collect();

    let _ = csv_add_row(log_path, &payload.email, &payload.id, "now").expect("Could not append row");
}

fn csv_add_row(path: PathBuf, email: &str, id: &str, timestamp: &str) -> Result<(), Box<dyn Error>> {

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let row = format!("{},{},{}", email, id, timestamp);
    writeln!(file, "{}", row)?;
    
    println!("Recorded attendance for {}", email);

    Ok(())
}
