use bech32::ToBase32;
use image::Luma;
use qrcode::QrCode;

pub fn create_lnurl_qrcode(url: &str, path: &str) {
    let encoded = bech32::encode("lnurl", url.as_bytes().to_base32()).unwrap();
    let code = QrCode::new(encoded.to_string()).unwrap();
    let image = code.render::<Luma<u8>>().build();
    image.save(path).unwrap();
}

fn main() {
    create_lnurl_qrcode("https://example.com","qrcode.png");
}
