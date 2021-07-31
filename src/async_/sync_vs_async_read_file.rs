const FILENAME: &str = ".gitignore";
const CONTENT: &str = "/target\nCargo.lock\na.out";
const READ_TIMES: usize = 100;

/// 468,530 ns read .gitignore 100 times
#[bench]
fn bench_async_io_uring_read_file(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        tokio_uring::start(async {
            for _ in 0..READ_TIMES {
                let mut output = Vec::with_capacity(CONTENT.len());
                let file = tokio_uring::fs::File::open(FILENAME).await.unwrap();
                let mut buf = vec![0_u8; CONTENT.len()];
                let mut pos = 0;
                loop {
                    let (n_read, read_chunk) = file.read_at(buf, pos).await;
                    let n_read = n_read.unwrap();
                    if n_read == 0 {
                        break;
                    }
                    output.extend_from_slice(&read_chunk[..n_read]);
                    pos += n_read as u64;
                    buf = read_chunk;
                }
                assert_eq!(unsafe { String::from_utf8_unchecked(output) }, CONTENT);
            }
        });
    });
}

/// 150,536 ns read .gitignore 100 times
#[bench]
fn bench_sync_single_thread_read_file(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        for _ in 0..READ_TIMES {
            assert_eq!(std::fs::read_to_string(FILENAME).unwrap(), CONTENT);
        }
    });
}

/// 1,529,080 ns read .gitignore 100 times
#[bench]
fn bench_sync_multi_thread_read_file(bencher: &mut test::Bencher) {
    bencher.iter(|| {
        let thread_handlers = (0..READ_TIMES)
            .map(|_| {
                std::thread::spawn(|| {
                    assert_eq!(std::fs::read_to_string(FILENAME).unwrap(), CONTENT);
                })
            })
            .collect::<Vec<_>>();
        for handler in thread_handlers {
            handler.join().unwrap();
        }
    });
}
