use qrcode_generator::{QRCodeError, QrCodeEcc};
use std::io::stdin;


fn main() {
    let mut input = String::new();
    println!("Enter the amount of banano you want");
    
    stdin().read_line(&mut input).expect("Failed to read line");

    let amount: f64 = input.trim().parse().expect("Please type the raw amount of banano you want");

    let mut input_address = String::new();
    println!("Enter the banano address you want");
    stdin().read_line(&mut input_address).expect("Failed to read line");

    let address = input_address.trim();


    qrcode_generator::to_png_to_file(format!("ban:{address}?amount={amount}"), QrCodeEcc::Low, 1024, "./image.png").unwrap();

}
