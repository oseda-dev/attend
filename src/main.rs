use std::error::Error;

use axum::{Router, routing::get};
use tokio::net::TcpListener;


mod qr;
mod backend;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {


    let path = qr::gen_qr_code("https://www.google.com")?;

    let backend_handle = tokio::spawn(backend::serve_backend());



    backend_handle.await?.expect("Server shut down");
    
    Ok(())
    
}
