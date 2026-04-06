use std::{error::Error, path::Path};

use axum::{Router, routing::get};
use tokio::net::TcpListener;


use std::env;

use crate::paths::mkdir_p;

mod qr;
mod backend;
mod paths;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // todo document me
    let key = "ATTEND_HOME";

    let binding = env::var(key)?;

    let attend_home = Path::new(&binding);


    let expanded = paths::expand_path(attend_home)?;
    mkdir_p(&expanded)?;

    println!("Expanded: {:?}", expanded);

    // let path = qr::gen_qr_code("https://www.google.com")?;

    // let backend_handle = tokio::spawn(backend::serve_backend());




    // backend_handle.await?.expect("Server shut downs");
    
    Ok(())
    
}


