use core::fmt;
use std::{collections::HashMap, error::Error, fs::OpenOptions, path::PathBuf};

use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::{get, post},
};
use std::io::Write;

use crate::{
    AttendFrontendState,
    templates::{self},
};

/// A request to log student attendance
///
/// # Fields
///
/// - `id` (`String`) - Unique-ish ID for student, probably a session ID
/// - `email` (`String`) - Student email.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let s = LogAttendanceRequest {
///     id: "UUID".to_string(),
///     email: "john.doe@university.com".to_string(),
/// };
/// ```
#[derive(serde::Deserialize, Debug)]
struct LogAttendanceRequest {
    id: String,
    email: String,
}

/// Network Socket
///
/// # Fields
///
/// - `ip` (`String`) - IP address.
/// - `port` (`u16`) - Network port.
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let s = Socket {
///     ip: "192.168.1.1".to_string(),
///     port: 3000,
/// };
/// ```
#[derive(Clone, Debug)]
pub struct Socket {
    pub ip: String,
    pub port: u16,
}

impl fmt::Display for Socket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

/// Serves the attend web server
///
/// # Arguments
///
/// - State(state): State<AttendFrontendState> - State from frontend, including ATTEND_HOME, class, and date.
///
/// # Returns
///
/// - `Result<(), Box<dyn Error + Send + Sync>>` - Ok(()) on positive server termination, propogating error. This is unlikely to return in normal use
///
pub async fn serve(state: &AttendFrontendState) -> Result<(), Box<dyn Error + Send + Sync>> {
    let app: Router = Router::new()
        .route("/", get(frontend))
        .route("/record", post(record_attendance))
        .with_state(state.clone());

    let addr = state
        .bind_socket
        .to_string()
        .parse::<std::net::SocketAddr>()?;
    let socket = tokio::net::TcpSocket::new_v4()?;

    socket.set_reuseaddr(true)?;

    socket.bind(addr)?;
    let listener = socket.listen(1024)?;

    axum::serve(listener, app).await?;

    Ok(())
}

/// Thin wrapper around `render_frontend`, since we are unable to return a result from `axum::route`` functions
///
/// # Arguments
///
/// - State(state): State<AttendFrontendState> - State from frontend, including ATTEND_HOME, class, and date.
///
async fn frontend(State(state): State<AttendFrontendState>) -> impl IntoResponse {
    match render_frontend(state.date, state.pub_socket) {
        Ok(html_content) => (StatusCode::OK, Html(html_content)).into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template Error: {}", err),
        )
            .into_response(),
    }
}

/// Renders the website to an HTML string
///
/// # Arguments
///
/// - `date` (`String`) - Current date (for attendance).
/// - `socket` (`Socket`) - Socket the server will post to. This is needed because the JS is dynamically generated
///
/// # Returns
///
/// - `Result<String, Box<dyn std::error::Error>>` - Ok(HTML string) on success, propogating the error
/// ```
fn render_frontend(date: String, socket: Socket) -> Result<String, Box<dyn std::error::Error>> {
    let template = templates::load_template()?;

    let mut replacements: HashMap<String, String> = HashMap::new();
    replacements.insert("DATE".to_owned(), date);
    replacements.insert("SOCKET".to_owned(), socket.to_string());

    let output = templates::render_template(template, replacements);

    Ok(output.to_string())
}

/// Records a students attendance from the the frontend
///
/// # Arguments
///
/// - State(state): State<AttendFrontendState> - State from frontend, including ATTEND_HOME, class, and date.
/// - Json(payload): Json<LogAttendanceRequest> - Payload of response
/// ```
async fn record_attendance(
    State(state): State<AttendFrontendState>,
    Json(payload): Json<LogAttendanceRequest>,
) {
    let log_path: PathBuf = [
        state.home_path,
        state.class.into(),
        state.date.into(),
        "log.csv".into(),
    ]
    .iter()
    .collect();

    csv_add_row(log_path, &payload.email, &payload.id, "now").expect("Could not append row");
}

/// Appends a row of parameters to the provided path
///
/// # Arguments
///
/// - `path` (`PathBuf`) - Path to csv
/// - `email` (`&str`) - Email of student.
/// - `id` (`&str`) - ID of student (usually a session ID).
/// - `timestamp` (`&str`) - Standard timestamp of log
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - Ok(()) on success, propogating the error
fn csv_add_row(
    path: PathBuf,
    email: &str,
    id: &str,
    timestamp: &str,
) -> Result<(), Box<dyn Error>> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    let row = format!("{},{},{}", email, id, timestamp);
    writeln!(file, "{}", row)?;

    println!("Recorded attendance for {}", email);

    Ok(())
}
