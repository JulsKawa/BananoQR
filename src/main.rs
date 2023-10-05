use qrcode_generator::{QRCodeError, QrCodeEcc};
use clap::Parser;


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
    let image = args.filename;

    qrcode_generator::to_png_to_file(format!("ban:{address}?amount={amount}"), QrCodeEcc::Low, 1024, format!("./{image}.png")).unwrap();

}
