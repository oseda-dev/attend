use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
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
mod duplicates;

/// Gets the value of the ATTEND_HOME env. variable, or the users home directory if not set
///
/// # Returns
///
/// - `Result<PathBuf, Box<dyn Error>>` - Ok(Path to the ATTEND_HOME directory), propagating error
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

/// Helper function to parse all class directory data to retrieve all valid class dates
/// and each student's set of attended dates.
fn load_attendance_data(class_path: &Path) -> Result<(BTreeSet<String>, BTreeMap<String, BTreeSet<String>>), Box<dyn Error>> {
    let mut all_days = BTreeSet::new();
    let mut student_attendance_days: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    for entry in fs::read_dir(class_path)? {
        let entry = entry?;
        let day_path = entry.path();

        if day_path.is_dir() {
            let log_file = day_path.join("log.csv");
            if log_file.exists() {
                if let Some(day_name) = day_path.file_name().and_then(|n| n.to_str()) {
                    let day_str = day_name.to_string();
                    all_days.insert(day_str.clone());

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
                        student_attendance_days
                            .entry(email)
                            .or_insert_with(BTreeSet::new)
                            .insert(day_str.clone());
                    }
                }
            }
        }
    }

    Ok((all_days, student_attendance_days))
}

/// Handler for the `check` subcommand, logging the overview table to stdout
fn handle_check() -> Result<(), Box<dyn Error>> {
    let home_path = get_attend_home()?;
    shell::mkdir_p(&home_path)?;
    shell::touch(&home_path.join("attend.conf"))?;

    let class = cli::prompt_class(&home_path)?;
    let class_path: PathBuf = home_path.join(class);

    let (all_days, student_attendance_days) = load_attendance_data(&class_path)?;
    let total_days = all_days.len();

    println!("\nAttendance Report for: {}", class_path.display());
    println!("Total Days Recorded: {}", total_days);
    println!("{:-<55}", "");
    println!(
        "{:<30} | {:<10} | {:<10}",
        "Student Email", "Attended", "Missed"
    );
    println!("{:-<55}", "");

    for (email, attended_dates) in &student_attendance_days {
        let attended_count = attended_dates.len();
        let missed_count = total_days - attended_count;
        println!(
            "{:<30} | {:<10} | {:<10}",
            email, attended_count, missed_count
        );
    }

    Ok(())
}

/// Handler for the `audit` subcommand, auditing a specific student's precise dates
fn handle_audit(email: String) -> Result<(), Box<dyn Error>> {
    let home_path = get_attend_home()?;
    shell::mkdir_p(&home_path)?;
    shell::touch(&home_path.join("attend.conf"))?;

    let class = cli::prompt_class(&home_path)?;
    let class_path: PathBuf = home_path.join(class);

    let (all_days, student_attendance_days) = load_attendance_data(&class_path)?;

    let target_clean = email.trim();
    println!("\n=================================================");
    println!("ATTENDANCE AUDIT FOR: {}", target_clean);
    println!("Classroom: {}", class_path.display());
    println!("=================================================");

    // default to empty if student never checkd in
    let empty_set = BTreeSet::new();
    let attended_dates = student_attendance_days.get(target_clean).unwrap_or(&empty_set);

    let missed_dates: Vec<&String> = all_days.difference(attended_dates).collect();

    //  handle attended
    println!("\n[Attended Days - {} total]:", attended_dates.len());
    if attended_dates.is_empty() {
        println!("  (None)");
    } else {
        for date in attended_dates {
            println!("  [X] {}", date);
        }
    }

    // handle missed
    println!("\n[Missed Days - {} total]:", missed_dates.len());
    if missed_dates.is_empty() {
        println!("  None");
    } else {
        for date in missed_dates {
            println!("  [ ] {}", date);
        }
    }

    Ok(())
}

/// Runs the attend application
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        let subcommand = &args[1];
        match subcommand.as_str() {
            "check" => {
                return handle_check();
            }
            "audit" => {
                if args.len() > 2 {
                    let email = args[2].clone();
                    return handle_audit(email);
                } else {
                    return Err("Err: Missing email address.\nUsage: cargo run -- audit [email]".into());
                }
            },
            "duplicates" => {
                return duplicates::handle_duplicates();
            }
            _ => {
                return Err(format!("Err: Unsupported Subcommand '{}'. Supported: 'check', 'audit'", subcommand).into());
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

