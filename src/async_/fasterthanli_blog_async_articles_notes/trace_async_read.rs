use futures::io::AsyncRead;
use std::pin::Pin;

/**
## 不用 async-trait 是因为弊端:
- 不能细粒度的控制 Send: 要么 ImplItem 里面所有方法全是 Send，要么全部全不是 Send，用 (?Send) 只是暂时解决该问题
- 不能细粒度的控制 Pin
*/
#[pin_project::pin_project]
struct TracingReader<R: AsyncRead> {
    #[pin]
    inner: R,
}

impl<R: AsyncRead> AsyncRead for TracingReader<R> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        // custom trace msg
        let address = &self as *const _;
        println!("{:?} => {:?}", address, std::thread::current().id());

        // inner origin AsyncRead
        // let inner: Pin<&mut R> = unsafe { self.map_unchecked_mut(|x| &mut x.inner) };
        let inner = self.project().inner;

        inner.poll_read(cx, buf)
    }
}
