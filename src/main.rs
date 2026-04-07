// todo document that you must do the arching yourself

use std::{
    collections::HashMap, error::Error, path::{Path, PathBuf}
};

use std::env;


use chrono::Local;
use inquire::{DateSelect, Select};
use std::fs::read_to_string;

mod backend;
mod paths;
mod qr;
mod cli;
mod shell;
mod templates;

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
    shell::mkdir_p(&home_path)?;


    // let path = qr::gen_qr_code("https://www.google.com")?;
    // let backend_handle = tokio::spawn(backend::serve_backend());

    let class = cli::prompt_class(&home_path)?;
    let date = cli::prompt_date()?;

    let selected_dir = home_path.join(class).join(date);

    shell::mkdir_p(&selected_dir)?;
    


    // let template_path = PathBuf::from("templates/index.html");
    let template = templates::load_template()?;
    
    let mut replacements: HashMap<String, String> = HashMap::new();
    replacements.insert("DATE".to_owned(), "2025".to_owned());
    let output = templates::render_template(template, replacements);

    println!("{:?}", output);

    // println!("template: {:?}", template);
    // backend_handle.await?.expect("Server shut downs");
    Ok(())
}
