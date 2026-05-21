use std::{
    collections::{BTreeMap, HashSet},
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use std::env;

use local_ip_address::local_ip;

use crate::web::Socket;

mod cli;
mod paths;
mod qr;
mod shell;
mod templates;
mod web;

/// Gets the value of the ATTEND_HOME env. variable, or the users home directory if not set
///
/// # Returns
///
/// - `Result<PathBuf, Box<dyn Error>>` - Ok(Path to the ATTEND_HOME directory), propogating error
///
fn get_attend_home() -> Result<PathBuf, Box<dyn Error>> {
    let key = "ATTEND_HOME";

    if let Ok(val) = env::var(key) {
        let expanded = paths::expand_path(Path::new(&val))?;
        return Ok(expanded);
    }

    // fall back to home dir
    let home_dir = dirs::home_dir().ok_or("Could not find user home directory")?;

    Ok(home_dir.join("Attend"))
}

/// All state necessary for rendering the frontend of the applications.
#[derive(Clone, Debug)]
pub struct AttendFrontendState {
    date: String,
    class: String,
    bind_socket: Socket,
    pub_socket: Socket,
    home_path: PathBuf,
}

/// Kills any process listening to a provided port number
///
/// # Platform
/// This function only works on Unix based systems
///
/// # Arguments
/// * `port_num` - the TCP port number to search for and terminate
///
/// # Returns
/// * `Ok(())` if processes were successfully terminated -> even if none were found
/// * `Err` if `lsof` or `kill` fails, or if output cannot be parsed properly
pub fn kill_port(port_num: u16) -> Result<(), Box<dyn Error>> {
    let lsof_out = Command::new("lsof")
        .arg("-t")
        .arg(format!("-i:{}", port_num))
        .output()?;

    let procs_on_port = String::from_utf8(lsof_out.stdout)?;

    for pid in procs_on_port.lines() {
        Command::new("kill").arg(pid).output()?;
    }

    Ok(())
}

/// Handler for the `check` subcommand, loggin output to stdout
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - Ok(()) on success, propogating error
///
fn handle_check() -> Result<(), Box<dyn Error>> {
    let home_path = get_attend_home()?;
    shell::mkdir_p(&home_path)?;
    shell::touch(&home_path.join("attend.conf"))?;

    let class = cli::prompt_class(&home_path)?;
    let class_path: PathBuf = home_path.join(class);

    // b tree map is basically just a sorted map
    let mut student_data: BTreeMap<String, usize> = BTreeMap::new();
    let mut total_days = 0;

    for entry in fs::read_dir(&class_path)? {
        let entry = entry?;
        let day_path = entry.path();

        if day_path.is_dir() {
            let log_file = day_path.join("log.csv");
            if log_file.exists() {
                total_days += 1;

                // hash set to ignore dupes
                let mut present_today = HashSet::new();
                let content = fs::read_to_string(log_file)?;

                for line in content.lines() {
                    let parts: Vec<&str> = line.split(',').collect();
                    if let Some(email) = parts.first()
                        && !email.trim().is_empty()
                    {
                        present_today.insert(email.trim().to_string());
                    }
                }

                for email in present_today {
                    *student_data.entry(email).or_insert(0) += 1;
                }
            }
        }
    }

    println!("\nAttendance Report for: {}", class_path.display());
    println!("Total Days Recorded: {}", total_days);
    println!("{:-<55}", "");
    println!(
        "{:<30} | {:<10} | {:<10}",
        "Student Email", "Attended", "Missed"
    );
    println!("{:-<55}", "");

    for (email, attended_count) in &student_data {
        let missed_count = total_days - attended_count;
        println!(
            "{:<30} | {:<10} | {:<10}",
            email, attended_count, missed_count
        );
    }

    Ok(())
}

/// Runs the attend application
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    if let Some(arg) = std::env::args().nth(1) {
        match arg.as_str() {
            "check" => return handle_check(),
            _ => {
                return Err("Err: Unsupported Argument".into());
            }
        }
    }

    let home_path = get_attend_home()?;
    shell::mkdir_p(&home_path)?;
    shell::touch(&home_path.join("attend.conf"))?;

    let class = cli::prompt_class(&home_path)?;
    let date = cli::prompt_date()?;

    // shell::mkdir_p(&home_path.join(class.clone()).join(date.clone()))?;
    let selected_dir = home_path.join(class.clone()).join(date.clone());

    shell::mkdir_p(&selected_dir)?;

    let port = 3000;

    kill_port(port)?;
    std::thread::sleep(std::time::Duration::from_millis(1000));

    let bind_socket = Socket {
        ip: "0.0.0.0".to_string(),
        port,
    };

    let pub_socket = Socket {
        ip: local_ip()?.to_string(),
        port,
    };

    let attend_state = AttendFrontendState {
        date,
        class,
        pub_socket: pub_socket.clone(),
        bind_socket,
        home_path,
    };

    let backend_handle = tokio::spawn(async move { web::serve(&attend_state).await });

    println!("Please visit {}", pub_socket);

    let qr_path = selected_dir.join("qr.png");
    qr::gen_qr_code(&qr_path, &format!("http://{}", &pub_socket.to_string()))?;
    open::that(qr_path)?;

    backend_handle.await?.expect("Server shut down");
    Ok(())
}
