// todo document that you must do the arching yourself

use std::{
    collections::HashMap, error::Error, path::{Path, PathBuf}
};

use std::env;


use chrono::Local;
use inquire::{DateSelect, Select};
use local_ip_address::linux::local_ip;
use std::fs::read_to_string;

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


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let home_path = get_attend_home()?;
    shell::mkdir_p(&home_path)?;


    // let path = qr::gen_qr_code("https://www.google.com")?;
    // let backend_handle = tokio::spawn(backend::serve_backend());

    let class = cli::prompt_class(&home_path)?;
    let date = cli::prompt_date()?;

    let selected_dir = home_path.join(class.clone()).join(date.clone());

    shell::mkdir_p(&selected_dir)?;
    
    // match web::serve().await {
    //     Ok(()) => {},
    //     Err(err) => {println!("{:?}", err)},
    // };
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

    let _ = qr::gen_qr_code(&format!("http://{}", &pub_socket.to_string()));
    // let template_path = PathBuf::from("templates/index.html");

    // println!("{:?}", output);

    // println!("template: {:?}", template);
    backend_handle.await?.expect("Server shut down");
    Ok(())
}
