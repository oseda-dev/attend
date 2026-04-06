use std::error::Error;


mod qr;

fn main() -> Result<(), Box<dyn Error>> {


    let path = qr::gen_qr_code("https://www.google.com")?;


    Ok(())
    
}
