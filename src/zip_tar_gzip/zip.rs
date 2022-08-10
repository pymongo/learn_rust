#[test]
fn zip() {
    dbg!(tree_magic::from_filepath(std::path::Path::new(
        "/home/w/Downloads/dragon-book-front-source5.zip"
    )));
    dbg!(tree_magic::from_filepath(std::path::Path::new(
        "/home/w/Downloads/dragon-book-front-source5.tar.gz"
    )));
    let mut zip = zip::ZipArchive::new(
        std::fs::File::open("/home/w/Downloads/dragon-book-front-source5.zip").unwrap(),
    )
    .unwrap();
    for i in 0..zip.len() {
        let file = zip.by_index(i).unwrap();
        println!("{}", file.name());
    }

    // let mut zip = zip::ZipArchive::new(
    //     std::fs::File::open("/home/w/Downloads/dragon-book-front-source5.tar.gz").unwrap(),
    // )
    // .unwrap();
    // for i in 0..zip.len() {
    //     let file = zip.by_index(i).unwrap();
    //     println!("Filename: {}", file.name());
    // }
}
