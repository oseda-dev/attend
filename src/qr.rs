use image::Luma;
use qrcode::QrCode;

use std::{error::Error, path::Path};

/// Generates a QR code from arbitrary data
///
/// # Arguments
///
/// - `path` (`&Path`) - Where to save the QR png file
/// - `data` (`&str`) - Data to write to QR code
///
/// # Returns
///
/// - `Result<(), Box<dyn Error>>` - Ok(()) on success, propogating error
///
pub fn gen_qr_code(path: &Path, data: &str) -> Result<(), Box<dyn Error>> {
    // path = str
    // pathbuf = String
    let code = QrCode::new(data)?;

    let image = code.render::<Luma<u8>>().build();

    image.save(&path)?;

    Ok(())
}
