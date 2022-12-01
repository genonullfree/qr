use qr_code::QrCode;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let msg = args[1..].join(" ");
    let qr = QrCode::new(msg.clone()).unwrap();
    println!("{}", qr.to_string(false, 3));
    println!("{}", msg);
}
