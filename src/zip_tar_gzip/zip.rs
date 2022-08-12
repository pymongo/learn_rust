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
        if file.is_dir() {
            println!("is_dir: {}", file.name());
        } else {
            println!("{}", file.name());
        }
    }
    println!("{}\n", "\n".repeat(5));
    // 以下遍历方式会是乱序，不一定是 文件夹-文件-子文件夹-子文件 这样规整
    // if file name is gbk encoding in windows would 乱码
    for each in zip.file_names() {
        println!("{}", each);
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
