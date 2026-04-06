use qrcode::QrCode;
use image::Luma;

use std::{error::Error, path::{Path, PathBuf}};

// path = str
// pathbuf = String
pub fn gen_qr_code(data: &str) -> Result<PathBuf, Box<dyn Error>> {
    let code = QrCode::new(data)?;
    
    let img_path = PathBuf::from("./img.png");

    let image = code.render::<Luma<u8>>().build();

    image.save(&img_path)?;

    return Ok(img_path);


}