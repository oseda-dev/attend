use std::{error::Error, fs::File, io::Read, path::{Path, PathBuf}};

use chrono::Local;
use inquire::DateSelect;

use crate::shell::touch;

use inquire::Select;

/// Prompts the user for a date from a date picker
/// 
/// # Returns
/// 
/// - `Result<String, Box<dyn Error>>` - Ok("%Y-%m-%d") date string, propogating error
/// 
pub fn prompt_date() -> Result<String, Box<dyn Error>> {
    // super fancy date selector. default to system times today
    let today = Local::now().naive_local().date();
    let date = DateSelect::new("Select a date (Enter for today)")
        .with_default(today)
        .prompt()?;

    Ok(date.format("%Y-%m-%d").to_string())
}

/// Prompt the user for a class from their classes in their ATTEND_HOME/attend.conf file
/// 
/// # Arguments
/// 
/// - `home` (`&Path`) - ATTEND_HOME value
/// 
/// # Returns
/// 
/// - `Result<String, Box<dyn Error>>` - Ok(class), propogating error.
/// 
pub fn prompt_class(home: &Path) -> Result<String, Box<dyn Error>> {
    let config_path = PathBuf::from(home).join("attend.conf");
    touch(&config_path)?;


    let mut conf_file = File::open(config_path)?;

    let mut buf = String::new();
    conf_file.read_to_string(&mut buf)?;

    let classes = buf
        .trim()
        .lines()
        .map(str::to_owned)
        .collect::<Vec<String>>();
    
    if classes.len() <= 0 {
        println!("It seems like you have no classes in your attend.conf file");
        println!("Please add some classes and try again");
        return Err("No classes found".into());
    }

    let choice = Select::new("Select class:", classes).prompt()?;

    Ok(choice)
}
