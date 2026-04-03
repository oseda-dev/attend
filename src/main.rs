use qrcode::QrCode;
use image::Luma;


fn main() {

    let code = QrCode::new(b"some data").unwrap();


    let image = code.render::<Luma<u8>>().build();

    image.save("img.png").expect("Image could not save");


    
}
