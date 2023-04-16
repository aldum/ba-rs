use image::Luma;
use qrcode::QrCode;

fn main() {
    // Encode some data into bits.
    let code = QrCode::new(b"01234567").unwrap();

    // let image = code.render::<Luma<u8>>().build();
    // image.save("/tmp/qrcode.png").unwrap();

    let string = code
        .render::<char>()
        .quiet_zone(false)
        .module_dimensions(2, 1)
        .build();
    println!("{}", string);
}
