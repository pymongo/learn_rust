

const BUF_SIZE: usize = 1 * 1024 * 1024;
const FILENAME_TO_READ: &str = "Cargo.toml";

fn read() {
    use std::io::Read;
    let mut buf = [0u8; BUF_SIZE];
    let mut f = std::fs::File::open(FILENAME_TO_READ).unwrap();
    f.read(&mut buf).unwrap();
    // println!("std::io::Read with {BUF_SIZE} no stack overflow");
}

async fn read_async() {
use tokio::io::AsyncReadExt;
let mut buf = [0u8; BUF_SIZE];
let mut f = tokio::fs::File::open(FILENAME_TO_READ).await.unwrap();
f.read(&mut buf).await.unwrap();
}

#[tokio::test(
    flavor = "multi_thread",
    worker_threads = 1
)]
async fn aa1() {
    read();
}

// #[tokio::test(
//     flavor = "multi_thread",
//     worker_threads = 1
// )]
#[tokio::test(
    flavor = "current_thread",
    // worker_threads = 1
)]
async fn aa2() {
    read_async().await;
}

use std::future::Future;
use tokio::sync::mpsc;
use futures::FutureExt;

pub trait OutputSize<A> {
    type Output;
}

// impl OutputSize for tokio::io::util::read::Read<'_, tokio::fs::File> {
// 
// }

macro_rules! impl_output_size {
    (($($ty:tt,)*)) => {
        impl<$($ty,)* F, R> OutputSize<($($ty,)*)> for F
        where
            F: Fn($($ty,)*) -> R ,
        {
            type Output = R;
        }
    };
}

impl_output_size!(());
impl_output_size!((A0,));
impl_output_size!((A0, A1,));
impl_output_size!((A0, A1, A2,));

#[inline]
#[must_use]
pub const fn output_size<F, A>(_: &F) -> usize
where
    F: OutputSize<A>,
{
    std::mem::size_of::<F::Output>()
}

struct Message([u8; 144]);

#[inline]
pub fn send<T: Send>(
    tx: &mpsc::Sender<T>,
    val: T,
) -> impl Future<Output = Result<(), T>> + Send + '_ {
    tx.reserve().map(move |result| match result {
        Ok(permit) => {
            permit.send(val);
            Ok(())
        }
        Err(_) => Err(val),
    })
}

#[test]
fn mpsc_future_size() {
    dbg!(output_size(&read_async));
    assert_eq!(std::mem::size_of::<Message>(), 144);
    assert_eq!(output_size(&send::<Message>), 240);
    assert_eq!(output_size(&mpsc::Sender::<Message>::send), 400);
}

fn open_file(path: &str) {
    // check path exists
        
}