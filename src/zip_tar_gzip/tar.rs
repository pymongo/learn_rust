//! https://rust-lang-nursery.github.io/rust-cookbook/compression/tar.html
//!

use flate2::read::GzDecoder;
use std::fs::File;
use tar::Archive;

#[test]
fn test_tar() {
    let tar_gz = File::open("/home/w/Downloads/dragon-book-front-source5.tar.gz").unwrap();
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    for entry in archive.entries().unwrap() {
        let entry = entry.unwrap();
        if entry.header().entry_type().is_dir() {
            println!("is_dir: {:?}", entry.path().unwrap());
        } else {
            println!("{:?}", entry.path().unwrap());
        }
    }
}
