#![feature(test)]
extern crate test;

// cargo +nightly bench --bench md5_digest -- --nocapture

//rust_md5::util::to_hex_string
const BYTES_STRING: &[u8] = "market_id=ethusdt&nonce=1589357159&price=100&public_key=49f29c25-f42b-4844-8247-5dc45af4b72f&source=api&volume=1".as_bytes();

/*
1,285 ns/iter (+/- 57)
1,057 ns/iter (+/- 123)
1,077 ns/iter (+/- 73)
*/

// #[bench]
// fn md5(bencher: &mut test::Bencher) {
//     bencher.iter(|| {
//         let digest_hex_string = format!("{:x}", md5::compute(BYTES_STRING));
//         assert_eq!("37899e89de5fdef8834d7b74e35d3c98", digest_hex_string);
//     });
// }

/*
370 ns/iter (+/- 61)
377 ns/iter (+/- 76)
367 ns/iter (+/- 48)
*/

// 结论md-5库性能最好
use md5::{Md5, Digest};
#[bench]
fn md_5(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let mut hasher = Md5::new();
        hasher.input(BYTES_STRING);
        let digest_hex_string = format!("{:x}", hasher.result());
        assert_eq!("37899e89de5fdef8834d7b74e35d3c98", digest_hex_string)
    })
}
