use flate2::write::{GzDecoder, GzEncoder};
use flate2::Compression;
use std::io::Write;

fn encode_str_to_gzip_bytes(s: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::fast());
    encoder.write_all(s.as_bytes()).unwrap();
    encoder.finish().unwrap()
}

fn decode_gzip_bytes_to_str(bytes: Vec<u8>) -> String {
    let mut writer = Vec::new();
    let mut decoder = GzDecoder::new(writer);
    decoder.write_all(&bytes[..]).unwrap();
    writer = decoder.finish().unwrap();
    String::from_utf8(writer).unwrap()
}

#[test]
fn main() {
    let s: &str = "apple";
    let gzip_bytes = encode_str_to_gzip_bytes(s);
    let decode_str = decode_gzip_bytes_to_str(gzip_bytes);
    println!("{}", decode_str);
    assert_eq!(decode_str.as_str(), s);
}
