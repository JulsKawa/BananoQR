use clap::Parser;
extern crate qrcode;
extern crate image;

use qrcode::QrCode;
use image::{DynamicImage, Rgba, RgbaImage, GenericImageView, imageops::{resize, overlay}};


fn generate_qr_code(data: &str) -> DynamicImage {
    // Generate the QR code
    let code = QrCode::new(data).unwrap();

    // Convert the QR code to an image
    let image = code.render::<Rgba<u8>>().build();

    image::DynamicImage::ImageRgba8(image)
}

fn load_logo(logo_path: &str) -> RgbaImage {
    image::open(logo_path)
        .expect("Failed to load logo image")
        .to_rgba8()
   
}

fn overlay_logo(qr_code: &DynamicImage, logo: &RgbaImage) -> DynamicImage {
    // Scale the logo to fit within the dimensions of the QR code
    let (qr_width, qr_height) = qr_code.dimensions();
    let logo = resize(logo, qr_width / 3, qr_height / 4, image::imageops::FilterType::Lanczos3);

    // Create a copy of the QR code image
    let mut combined_image = qr_code.clone();

    // Calculate the position to center the logo on the QR code
    let x_offset = (qr_width - logo.width()) / 2;
    let y_offset = (qr_height - logo.height()) / 2;

    // Overlay the logo on top of the QR code
    overlay(&mut combined_image, &logo, x_offset, y_offset);

    combined_image
}

fn save_qr_code_with_logo(image: &DynamicImage, output_path: &str) {
    image.save(output_path).expect("Failed to save image");
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct BananoQR {
    amount: f64,
    address: String,
    filename: String,
}

 fn main() {
    let args = BananoQR::parse();

    let address = args.address;
    let amount = args.amount;
    let filename = args.filename;

    let data = format!("ban:{address}?amount={amount}"); // Replace with your QR code data
    let logo_path = "logo.png"; // Replace with the path to your logo image
    let output_path = format!("{filename}.png"); // Replace with the desired output path

    let qr_code = generate_qr_code(&data);
    let logo = load_logo(logo_path);
    let qr_code_with_logo = overlay_logo(&qr_code, &logo);
    save_qr_code_with_logo(&qr_code_with_logo, &output_path);

}