use qrcode_generator::QrCodeEcc;

pub async fn generate_qrcode(brcode: String) {
    qrcode_generator::to_png_to_file(&brcode, QrCodeEcc::Low, 1024, "brcode.png").unwrap();
}
