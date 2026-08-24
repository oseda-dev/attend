use std::{
    collections::HashMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use crate::{cli, get_attend_home, shell};
use std::collections::HashSet;

/// Helper function to scan all log.csv files for session IDs shared across different students.
/// Returns a map of: Session ID -> Vec<(Date, Email)>
fn find_cross_student_duplicate_sessions(
    class_path: &Path,
) -> Result<HashMap<String, Vec<(String, String)>>, Box<dyn Error>> {
    // Session ID -> Vec<(Date, Email)>
    let mut session_map: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for entry in fs::read_dir(class_path)? {
        let entry = entry?;
        let day_path = entry.path();

        if day_path.is_dir() {
            let log_file = day_path.join("log.csv");
            if log_file.exists() {
                if let Some(day_name) = day_path.file_name().and_then(|n| n.to_str()) {
                    let day_str = day_name.to_string();
                    let content = fs::read_to_string(log_file)?;

                    for line in content.lines() {
                        let parts: Vec<&str> = line.split(',').collect();
                        if parts.len() >= 2 {
                            let email = parts[0].trim().to_string();
                            let session_id = parts[1].trim().to_string();

                            if !session_id.is_empty() && !email.is_empty() {
                                session_map
                                    .entry(session_id)
                                    .or_default()
                                    .push((day_str.clone(), email));
                            }
                        }
                    }
                }
            }
        }
    }

    // filter to retain ONLY session IDs used by 2 (or more) separate student emails
    let suspicious_duplicates = session_map
        .into_iter()
        .filter(|(_, occurrences)| {
            let unique_emails: HashSet<&String> =
                occurrences.iter().map(|(_, email)| email).collect();
            unique_emails.len() > 1
        })
        .collect();

    Ok(suspicious_duplicates)
}

/// Handler for the `duplicates` subcommand
pub fn handle_duplicates() -> Result<(), Box<dyn Error>> {
    let home_path = get_attend_home()?;
    shell::mkdir_p(&home_path)?;
    shell::touch(&home_path.join("attend.conf"))?;

    let class = cli::prompt_class(&home_path)?;
    let class_path: PathBuf = home_path.join(class);

    let duplicates = find_cross_student_duplicate_sessions(&class_path)?;

    println!("\n=================================================");
    println!("Duplicate Session Report:");
    println!("Classroom: {}", class_path.display());
    println!("=================================================");

    if duplicates.is_empty() {
        println!("\nNo session IDs were shared between different students!");
    } else {
        println!(
            "\nFound {} session ID(s) shared by MULTIPLE students:\n",
            duplicates.len()
        );
        for (session_id, occurrences) in duplicates {
            println!("Session ID: {}", session_id);
            for (date, email) in occurrences {
                println!("  - Date: {:<12} | Email: {}", date, email);
            }
            println!();
        }
    }

    Ok(())
}
