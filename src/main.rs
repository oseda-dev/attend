// todo document that you must do the arching yourself

use std::{
    collections::{BTreeMap, HashSet}, error::Error, fs, path::{Path, PathBuf}
};

use std::env;


use local_ip_address::local_ip;

use crate::web::Socket;

mod web;
mod paths;
mod qr;
mod cli;
mod shell;
mod templates;

// todo document this
fn get_attend_home() -> Result<PathBuf, Box<dyn Error>> {
    let key = "ATTEND_HOME";

    if let Ok(val) = env::var(key) {
        let expanded = paths::expand_path(Path::new(&val))?;
        return Ok(expanded);
    }

    // fall back to home dir
    let home_dir = dirs::home_dir()
        .ok_or("Could not find user home directory")?;

    Ok(home_dir.join("Attend"))
}

#[derive(Clone, Debug)]
pub struct AttendFrontendState {
    date: String,
    class: String,
    bind_socket: Socket,
    pub_socket: Socket,
    home_path: PathBuf
}

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
                    if let Some(email) = parts.first() {
                        if !email.trim().is_empty() {
                            present_today.insert(email.trim().to_string());
                        }
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
    println!("{:<30} | {:<10} | {:<10}", "Student Email", "Attended", "Missed");
    println!("{:-<55}", "");

    for (email, attended_count) in &student_data {
        let missed_count = total_days - attended_count;
        println!("{:<30} | {:<10} | {:<10}", email, attended_count, missed_count);
    }

    Ok(())
}


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

    let selected_dir = home_path.join(class.clone()).join(date.clone());

    shell::mkdir_p(&selected_dir)?;
    
    let port = 3000;

    let bind_socket = Socket{
        ip: "0.0.0.0".to_string(),
        port: port
    };

    let pub_socket = Socket{
        ip: local_ip()?.to_string(),
        port: port
    };

    let attend_state = AttendFrontendState { 
        date: date,
        class: class,
        pub_socket: pub_socket.clone(),
        bind_socket: bind_socket,
        home_path: home_path,
    };

    let backend_handle = tokio::spawn(async move { 
        web::serve(&attend_state).await
    });


    println!("Please visit {}", pub_socket);

    let qr_path = selected_dir.join("qr.png");
    qr::gen_qr_code(&qr_path, &format!("http://{}", &pub_socket.to_string()))?;
    open::that(qr_path)?;

    backend_handle.await?.expect("Server shut down");
    Ok(())
}
