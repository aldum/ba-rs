use std::env;
use std::io;

use qrcodegen::QrCode;
use qrcodegen::QrCodeEcc;

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(args);

    let stdin = read_stdin().unwrap();

    let data: &str = &stdin;
    let errcorlvl: QrCodeEcc = QrCodeEcc::Low;

    let qr = QrCode::encode_text(data, errcorlvl).unwrap();
    print_qr(&qr);
}

fn read_stdin() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer)
}

fn print_qr(qr: &QrCode) {
    let border: i32 = 4;
    for y in -border..qr.size() + border {
        for x in -border..qr.size() + border {
            let c: char = if qr.get_module(x, y) { '█' } else { ' ' };
            print!("{0}{0}", c);
        }
        println!();
    }
    println!();
}
