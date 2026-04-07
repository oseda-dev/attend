use std::{
    error::Error,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use std::env;

use crate::{
    paths::{expand_path, mkdir_p},
    shell::touch,
};

use chrono::Local;
use inquire::{DateSelect, Select};
use std::fs::read_to_string;

mod backend;
mod paths;
mod qr;
mod shell;

fn get_attend_home() -> Result<PathBuf, Box<dyn Error>> {
    // todo document me
    let key = "ATTEND_HOME";

    let binding = env::var(key)?;

    let attend_home = Path::new(&binding);
    let expanded = paths::expand_path(attend_home)?;

    Ok(expanded)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let home_path = get_attend_home()?;
    mkdir_p(&home_path)?;


    // let path = qr::gen_qr_code("https://www.google.com")?;
    // let backend_handle = tokio::spawn(backend::serve_backend());

    let class = prompt_class(&home_path)?;
    let date = prompt_date()?;
    
    // backend_handle.await?.expect("Server shut downs");
    Ok(())
}

fn prompt_date() -> Result<String, Box<dyn Error>> {
    // super fancy date selector. default to system times today
    let today = Local::now().naive_local().date();
    let date = DateSelect::new("Select a date (Enter for today)")
        .with_default(today)
        .prompt()?;

    Ok(date.format("%Y-%m-%d").to_string())
}

fn prompt_class(home: &Path) -> Result<String, Box<dyn Error>> {
    let config_path = PathBuf::from(home).join("attend.conf");
    touch(&config_path)?;

    let mut conf_file = File::open(config_path)?;

    let mut buf = String::new();
    conf_file.read_to_string(&mut buf)?;

    let classes = buf
        .trim()
        .split("\n")
        .map(str::to_owned)
        .collect::<Vec<String>>();
    
    let choice = Select::new("Select class:", classes).prompt()?;

    Ok(choice)
}
