use qrcode::QrCode;
use image::Luma;

use std::{error::Error, path::Path};

// path = str
// pathbuf = String
pub fn gen_qr_code(path: &Path, data: &str) -> Result<(), Box<dyn Error>> {
    let code = QrCode::new(data)?;
    
    let image = code.render::<Luma<u8>>().build();

    image.save(&path)?;

    Ok(())
}